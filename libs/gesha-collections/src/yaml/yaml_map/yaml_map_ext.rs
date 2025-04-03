use crate::partial_result::PartialResult;
use crate::tracking::TrackingKeyAppendable;
use crate::yaml::{Converter, Extractor, YamlError, YamlMap, YamlValue};
use std::marker::PhantomData;

pub trait YamlMapExt<E>
where
    E: From<YamlError>,
{
    fn extractor<'a, F, X, Y>(&'a mut self, key: &'a str, f: F) -> Extractor<'a, F, X, Y, E>
    where
        F: FnOnce(X) -> Y,
        X: TryFrom<YamlValue, Error = YamlError>;

    fn extract<F, X, Y, Z>(&mut self, key: &str, f: F) -> Z
    where
        F: FnOnce(X) -> Y,
        F: Converter<Result<X, E>, Y, Z>,
        X: TryFrom<YamlValue, Error = YamlError>,
        Z: TrackingKeyAppendable,
    {
        self.extractor(key, f).as_required()
    }

    fn extract_if_exists<F, X, Y, Z>(&mut self, key: &str, f: F) -> Z
    where
        F: FnOnce(X) -> Y,
        F: Converter<Result<Option<X>, E>, Y, Z>,
        X: TryFrom<YamlValue, Error = YamlError>,
        Z: TrackingKeyAppendable,
    {
        self.extractor(key, f).as_optional()
    }

    fn extract_with_default<F, X, Y, Z>(&mut self, key: &str, f: F) -> Z
    where
        F: FnOnce(X) -> PartialResult<Y, E>,
        F: Converter<PartialResult<X, E>, PartialResult<Y, E>, Z>,
        X: TryFrom<YamlValue, Error = YamlError>,
        X: Default,
        Z: TrackingKeyAppendable,
    {
        self.extractor(key, f).as_required_with_default()
    }

    fn error_if_exists<A, F>(&mut self, key: &str, f: F) -> Result<(), E>
    where
        A: TryFrom<YamlValue, Error = YamlError>,
        F: FnOnce(A) -> E,
    {
        self.extractor(key, f).error_if_exists()
    }
}

impl<E> YamlMapExt<E> for YamlMap
where
    E: From<YamlError>,
{
    fn extractor<'a, F, X, Y>(&'a mut self, key: &'a str, converter: F) -> Extractor<'a, F, X, Y, E>
    where
        F: FnOnce(X) -> Y,
        X: TryFrom<YamlValue, Error = YamlError>,
    {
        Extractor {
            map: self,
            key,
            converter,
            _phantom: PhantomData,
        }
    }
}
