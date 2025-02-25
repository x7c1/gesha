use crate::yaml::YamlMap;
use crate::{Output, Result};

/// convert YamlMap to a type defined in this crate.
pub trait ToOpenApi: Sized {
    fn apply(map: YamlMap) -> Result<Output<Self>>;
}
