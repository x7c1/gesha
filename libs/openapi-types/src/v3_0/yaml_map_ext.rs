use crate::{Error, Output, Result, by_key, v3_0, with_key};
use gesha_collections::partial_result::{MaybeOps, MergeOps};
use gesha_collections::yaml::{YamlError, YamlMap, YamlValue};
use std::fmt::Display;
use std::marker::PhantomData;
use v3_0::SpecViolation::{FieldNotExist, TypeMismatch};

pub struct Extractor<'a, F, X, Y>
where
    F: FnOnce(X) -> Y,
    X: TryFrom<YamlValue, Error = YamlError>,
{
    map: &'a mut YamlMap,
    key: &'a str,
    f: F,
    _phantom: PhantomData<(X, Y)>,
}

#[allow(clippy::wrong_self_convention)]
impl<F, X, Y> Extractor<'_, F, X, Y>
where
    F: FnOnce(X) -> Y,
    X: TryFrom<YamlValue, Error = YamlError>,
{
    pub fn as_required<Z>(self) -> Z
    where
        F: Converter<Result<X>, Y, Z>,
        Z: TrackingKeyAppendable,
    {
        let result = self.map.remove(self.key).map_err(to_crate_error);
        self.f.convert(result).append(self.key)
    }

    pub fn as_optional<Z>(self) -> Z
    where
        F: Converter<Result<Option<X>>, Y, Z>,
        Z: TrackingKeyAppendable,
    {
        let result = self.map.remove_if_exists(self.key).map_err(to_crate_error);
        self.f.convert(result).append(self.key)
    }

    pub fn as_required_with_default<Z>(self) -> Z
    where
        F: Converter<Output<X>, Y, Z>,
        X: Default,
        Z: TrackingKeyAppendable,
    {
        let result: Output<X> = self
            .map
            .remove_if_exists::<X>(self.key)
            .map_err(to_crate_error)
            .maybe()
            .map(|maybe| maybe.unwrap_or_default());

        self.f.convert(result).append(self.key)
    }

    pub fn error_if_exists(self) -> Result<()>
    where
        Y: Into<Error>,
    {
        let maybe = self
            .map
            .remove_if_exists::<X>(self.key)
            .map_err(to_crate_error)?;

        let Some(x) = maybe else {
            return Ok(());
        };
        Err((self.f)(x).into())
    }
}

pub trait Converter<X, Y, Z> {
    fn convert(self, x: X) -> Z;
}

impl<F, X, Z> Converter<Output<X>, Output<Z>, Output<Z>> for F
where
    F: FnOnce(X) -> Output<Z>,
{
    fn convert(self, x: Output<X>) -> Output<Z> {
        x.map(self).flatten()
    }
}

impl<F, X, Z> Converter<Result<X>, Result<Z>, Result<Z>> for F
where
    F: FnOnce(X) -> Result<Z>,
{
    fn convert(self, x: Result<X>) -> Result<Z> {
        x.and_then(self)
    }
}

impl<F, X, Z> Converter<Result<X>, Output<Z>, Result<Output<Z>>> for F
where
    F: FnOnce(X) -> Output<Z>,
{
    fn convert(self, x: Result<X>) -> Result<Output<Z>> {
        x.map(self)
    }
}

impl<F, X, Z> Converter<Result<Option<X>>, Output<Z>, Output<Option<Z>>> for F
where
    F: FnOnce(X) -> Output<Z>,
{
    fn convert(self, x: Result<Option<X>>) -> Output<Option<Z>> {
        x.maybe().map(|maybe_a| maybe_a.map(self).maybe()).flatten()
    }
}

impl<F, X, Z> Converter<Result<Option<X>>, Result<Z>, Output<Option<Z>>> for F
where
    F: FnOnce(X) -> Result<Z>,
{
    fn convert(self, x: Result<Option<X>>) -> Output<Option<Z>> {
        x.maybe().map(|maybe_a| maybe_a.map(self).maybe()).flatten()
    }
}

impl<F, X, Z> Converter<Result<Option<X>>, Result<Output<Z>>, Output<Option<Z>>> for F
where
    F: FnOnce(X) -> Result<Output<Z>>,
{
    fn convert(self, result: Result<Option<X>>) -> Output<Option<Z>> {
        let x = match result {
            Ok(Some(x)) => x,
            Ok(None) => return Output::new(None, vec![]),
            Err(e) => return Output::new(None, vec![e]),
        };
        let output = match self(x) {
            Ok(output) => output,
            Err(e) => return Output::new(None, vec![e]),
        };
        output.map(Some)
    }
}

pub trait TrackingKeyAppendable {
    fn append(self, key: &str) -> Self;
}

impl<A> TrackingKeyAppendable for Result<A> {
    fn append(self, key: &str) -> Self {
        self.map_err(by_key(key))
    }
}

impl<A> TrackingKeyAppendable for Output<A> {
    fn append(self, key: &str) -> Self {
        self.bind_errors(with_key(key))
    }
}

pub trait YamlMapExt {
    fn extractor<'a, F, X, Y>(&'a mut self, key: &'a str, f: F) -> Extractor<'a, F, X, Y>
    where
        F: FnOnce(X) -> Y,
        X: TryFrom<YamlValue, Error = YamlError>;

    fn extract<F, X, Y, Z>(&mut self, key: &str, f: F) -> Z
    where
        F: FnOnce(X) -> Y,
        X: TryFrom<YamlValue, Error = YamlError>,
        F: Converter<Result<X>, Y, Z>,
        Z: TrackingKeyAppendable,
    {
        self.extractor(key, f).as_required()
    }

    fn extract_if_exists<F, X, Y, Z>(&mut self, key: &str, f: F) -> Z
    where
        F: FnOnce(X) -> Y,
        X: TryFrom<YamlValue, Error = YamlError>,
        F: Converter<Result<Option<X>>, Y, Z>,
        Z: TrackingKeyAppendable,
    {
        self.extractor(key, f).as_optional()
    }

    fn extract_with_default<F, X, Y, Z>(&mut self, key: &str, f: F) -> Z
    where
        F: FnOnce(X) -> Output<Y>,
        X: TryFrom<YamlValue, Error = YamlError>,
        X: Default,
        F: Converter<Output<X>, Output<Y>, Z>,
        Z: TrackingKeyAppendable,
    {
        self.extractor(key, f).as_required_with_default()
    }

    fn error_if_exists<A, E, F>(&mut self, key: &str, f: F) -> Result<()>
    where
        A: TryFrom<YamlValue, Error = YamlError>,
        E: Into<Error>,
        F: FnOnce(A) -> E,
    {
        self.extractor(key, f).error_if_exists()
    }
}

impl YamlMapExt for YamlMap {
    fn extractor<'a, F, X, Y>(&'a mut self, key: &'a str, f: F) -> Extractor<'a, F, X, Y>
    where
        F: FnOnce(X) -> Y,
        X: TryFrom<YamlValue, Error = YamlError>,
    {
        Extractor {
            map: self,
            key,
            f,
            _phantom: PhantomData,
        }
    }
}

pub fn reify_value<A>(v: std::result::Result<YamlValue, YamlError>) -> Result<A>
where
    A: TryFrom<YamlValue, Error = YamlError>,
{
    v.map_err(to_crate_error)?
        .try_into()
        .map_err(to_crate_error)
}

pub fn collect<X, Y, F>(f: F) -> impl FnOnce(YamlMap) -> Output<Y>
where
    F: Fn((String, YamlMap)) -> Result<Output<X>>,
    Y: FromIterator<X>,
{
    |map| {
        let (pairs, errors1) = {
            let init = (vec![], vec![]);
            fold(init, map.into_iter(), reify_entry)
        };
        let (xs, errors2) = {
            let init = (vec![], errors1);
            fold(init, pairs.into_iter(), f)
        };
        let output = xs.merge().append(errors2);
        output.map(|xs| xs.into_iter().collect())
    }
}

fn reify_entry<A, B>(kv: std::result::Result<(YamlValue, YamlValue), YamlError>) -> Result<(A, B)>
where
    A: TryFrom<YamlValue, Error = YamlError> + Display,
    B: TryFrom<YamlValue, Error = YamlError>,
{
    let (k, v) = kv.map_err(to_crate_error)?;
    let outline = k.outline();
    let key: A = k
        .try_into()
        .map_err(to_crate_error)
        .map_err(by_key(outline))?;

    let cloned = key.to_string();
    let value = v
        .try_into()
        .map_err(to_crate_error)
        .map_err(by_key(cloned))?;

    Ok((key, value))
}

fn fold<X, Y, T, F>(init: (Vec<Y>, Vec<Error>), iter: T, f: F) -> (Vec<Y>, Vec<Error>)
where
    T: Iterator<Item = X>,
    F: Fn(X) -> Result<Y>,
{
    iter.map(f).fold(init, |(mut xs, mut errors), y| {
        match y {
            Ok(x) => xs.push(x),
            Err(e) => errors.push(e),
        }
        (xs, errors)
    })
}

fn to_crate_error(e: YamlError) -> Error {
    match e {
        YamlError::FieldNotExist { field } => {
            Error::SpecViolation(crate::SpecViolation::from(FieldNotExist { field }))
        }
        YamlError::TypeMismatch { found, expected } => {
            Error::SpecViolation(crate::SpecViolation::from(TypeMismatch { found, expected }))
        }
        YamlError::UnknownType { found } => {
            Error::Unsupported(crate::Unsupported::UnknownType { found })
        }
    }
}
