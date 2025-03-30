use crate::core::{OutputMergeOps, OutputOptionOps};
use crate::error::by_key;
use crate::yaml::{YamlError, YamlMap, YamlValue};
use crate::{Error, Output, Result, v3_0, with_key};
use std::fmt::Display;
use v3_0::SpecViolation::{FieldNotExist, TypeMismatch};

pub trait YamlExtractor {
    fn extract<A>(&mut self, key: &str) -> Result<A>
    where
        A: TryFrom<YamlValue, Error = YamlError>;

    fn extract_or_by_default<F, A, B>(&mut self, key: &str, f: F) -> Output<B>
    where
        F: FnOnce(A) -> Output<B>,
        A: TryFrom<YamlValue, Error = YamlError>,
        A: Default;

    fn transform<F, A, B, T>(&mut self, key: &str, f: F) -> Result<Output<B>>
    where
        F: FnOnce(A) -> T,
        A: TryFrom<YamlValue, Error = YamlError>,
        F: MayTransform<A, B, T>;

    fn transform_if_exists<F, A, B, T>(&mut self, key: &str, f: F) -> Output<Option<B>>
    where
        F: FnOnce(A) -> T,
        F: CanTransform<A, B, T>,
        A: TryFrom<YamlValue, Error = YamlError>;

    fn extract_if_exists<A>(&mut self, key: &str) -> Output<Option<A>>
    where
        A: TryFrom<YamlValue, Error = YamlError>;
}

impl YamlExtractor for YamlMap {
    fn extract<A>(&mut self, key: &str) -> Result<A>
    where
        A: TryFrom<YamlValue, Error = YamlError>,
    {
        self.remove(key).map_err(to_crate_error)
    }

    fn extract_or_by_default<F, A, B>(&mut self, key: &str, f: F) -> Output<B>
    where
        F: FnOnce(A) -> Output<B>,
        A: TryFrom<YamlValue, Error = YamlError>,
        A: Default,
    {
        self.remove::<A>(key)
            .map_err(to_crate_error)
            .maybe()
            .map(|maybe| maybe.unwrap_or_default())
            .map(f)
            .flatten()
            .bind_errors(with_key(key))
    }

    fn transform<F, A, B, T>(&mut self, key: &str, f: F) -> Result<Output<B>>
    where
        F: FnOnce(A) -> T,
        F: MayTransform<A, B, T>,
        A: TryFrom<YamlValue, Error = YamlError>,
    {
        self.remove::<A>(key)
            .map_err(to_crate_error)
            .and_then(|a| f.apply(a))
            .map(|output| output.bind_errors(with_key(key)))
    }

    fn transform_if_exists<F, A, B, T>(&mut self, key: &str, f: F) -> Output<Option<B>>
    where
        F: FnOnce(A) -> T,
        F: CanTransform<A, B, T>,
        A: TryFrom<YamlValue, Error = YamlError>,
    {
        self.remove_if_exists::<A>(key)
            .map_err(to_crate_error)
            .maybe()
            .map(|a| f.apply(a))
            .flatten()
            .bind_errors(with_key(key))
    }

    fn extract_if_exists<A>(&mut self, key: &str) -> Output<Option<A>>
    where
        A: TryFrom<YamlValue, Error = YamlError>,
    {
        self.remove_if_exists::<A>(key)
            .map_err(to_crate_error)
            .maybe()
            .bind_errors(with_key(key))
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

pub trait MayTransform<A, B, T> {
    fn apply(self, a: A) -> Result<Output<B>>;
}

impl<A, B, F> MayTransform<A, B, Result<Output<B>>> for F
where
    F: FnOnce(A) -> Result<Output<B>>,
{
    fn apply(self, a: A) -> Result<Output<B>> {
        self(a)
    }
}

impl<A, B, F> MayTransform<A, B, Output<B>> for F
where
    F: FnOnce(A) -> Output<B>,
{
    fn apply(self, a: A) -> Result<Output<B>> {
        Ok(self(a))
    }
}

impl<A, B, F> MayTransform<A, B, Result<B>> for F
where
    F: FnOnce(A) -> Result<B>,
{
    fn apply(self, a: A) -> Result<Output<B>> {
        self(a).map(Output::ok)
    }
}

pub trait CanTransform<A, B, T> {
    fn apply(self, a: Option<A>) -> Output<Option<B>>;
}

impl<A, B, F> CanTransform<A, B, Result<Output<B>>> for F
where
    F: FnOnce(A) -> Result<Output<B>>,
{
    fn apply(self, a: Option<A>) -> Output<Option<B>> {
        a.map(self).maybe().map(|x| x.maybe()).flatten()
    }
}

impl<A, B, F> CanTransform<A, B, Result<B>> for F
where
    F: FnOnce(A) -> Result<B>,
{
    fn apply(self, a: Option<A>) -> Output<Option<B>> {
        a.map(self).maybe()
    }
}

impl<A, B, F> CanTransform<A, B, Output<B>> for F
where
    F: FnOnce(A) -> Output<B>,
{
    fn apply(self, a: Option<A>) -> Output<Option<B>> {
        a.map(self).maybe()
    }
}
