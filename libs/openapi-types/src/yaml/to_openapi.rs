use crate::Output;
use crate::yaml::YamlMap;

/// convert YamlMap to a type defined in this crate.
pub trait ToOpenApi: Sized {
    fn apply(map: YamlMap) -> Output<Self>;
}
