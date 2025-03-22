use crate::define_yaml_extractor;
use crate::v3_0::SpecViolation;

define_yaml_extractor!(SpecViolation);

// TODO: remove this
// use crate::core::OutputOptionOps;
// use crate::error::{Unsupported, with_key};
// use crate::yaml::{YamlError, YamlMap, YamlValue};
// use crate::{Error, Output, SpecViolation, v3_0, define_yaml_extractor};
//
// pub trait YamlExtractor<A>
// where
//     A: TryFrom<YamlValue, Error = YamlError>,
// {
//     fn extract_if_exists2(&mut self, key: &str) -> Output<Option<A>>;
// }
//
// impl<A> YamlExtractor<A> for YamlMap
// where
//     A: TryFrom<YamlValue, Error = YamlError>,
// {
//     fn extract_if_exists2(&mut self, key: &str) -> Output<Option<A>> {
//         let result = self.remove_if_exists2::<A>(key);
//         let result = result.map_err(to_crate_error);
//         let maybe = result.maybe();
//         maybe.bind_errors(with_key(key))
//     }
// }
//
// fn to_crate_error(e: YamlError) -> Error {
//     match e {
//         YamlError::TypeMismatch { found, expected } => {
//             Error::SpecViolation(SpecViolation::from(v3_0::SpecViolation::TypeMismatch {
//                 found,
//                 expected,
//             }))
//         }
//         YamlError::UnknownType { found } => Error::Unsupported(Unsupported::UnknownType { found }),
//     }
// }
