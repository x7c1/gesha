use crate::partial_result::{MaybeOps, PartialResult};
use crate::yaml::{Converter, TrackingKeyAppendable, YamlError, YamlMap, YamlValue};
use std::marker::PhantomData;

pub struct Extractor<'a, F, X, Y, E>
where
    F: FnOnce(X) -> Y,
    E: From<YamlError>,
    X: TryFrom<YamlValue, Error = YamlError>,
{
    pub(crate) map: &'a mut YamlMap,
    pub(crate) key: &'a str,
    pub(crate) converter: F,
    pub(crate) _phantom: PhantomData<(X, Y, E)>,
}

#[allow(clippy::wrong_self_convention)]
impl<F, X, Y, E> Extractor<'_, F, X, Y, E>
where
    F: FnOnce(X) -> Y,
    E: From<YamlError>,
    X: TryFrom<YamlValue, Error = YamlError>,
{
    pub fn as_required<Z>(self) -> Z
    where
        F: Converter<Result<X, E>, Y, Z>,
        Z: TrackingKeyAppendable,
    {
        let result = self.map.remove(self.key).map_err(E::from);
        self.converter.convert(result).with_key(self.key)
    }

    pub fn as_optional<Z>(self) -> Z
    where
        F: Converter<Result<Option<X>, E>, Y, Z>,
        Z: TrackingKeyAppendable,
    {
        let result = self.map.remove_if_exists(self.key).map_err(E::from);
        self.converter.convert(result).with_key(self.key)
    }

    pub fn as_required_with_default<Z>(self) -> Z
    where
        F: Converter<PartialResult<X, E>, Y, Z>,
        X: Default,
        Z: TrackingKeyAppendable,
    {
        let result: PartialResult<X, E> = self
            .map
            .remove_if_exists::<X>(self.key)
            .map_err(E::from)
            .maybe()
            .map(|maybe| maybe.unwrap_or_default());

        self.converter.convert(result).with_key(self.key)
    }

    pub fn error_if_exists(self) -> Result<(), E>
    where
        Y: Into<E>,
    {
        let maybe = self.map.remove_if_exists::<X>(self.key).map_err(E::from)?;

        let Some(x) = maybe else {
            return Ok(());
        };
        Err((self.converter)(x).into())
    }
}
