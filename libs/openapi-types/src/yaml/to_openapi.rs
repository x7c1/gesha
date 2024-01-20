use crate::yaml::YamlMap;
use crate::Result;

/// convert YamlMap to a type defined in this crate.
pub trait ToOpenApi: Sized {
    fn apply(map: YamlMap) -> Result<Self>;
}
