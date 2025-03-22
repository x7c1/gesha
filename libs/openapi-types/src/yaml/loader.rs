use crate::yaml::loader::YamlLoaderError::Conversion;
use crate::yaml::{YamlError, YamlValue};
use crate::{Error, Result};
use YamlLoaderError::CannotScanYaml;
use std::fmt::Debug;
use yaml_rust::YamlLoader;

pub fn load_from_str<A>(contents: &str) -> Result<A>
where
    A: TryFrom<YamlValue, Error = YamlError>,
{
    let mut yamls = YamlLoader::load_from_str(contents).map_err(|e| CannotScanYaml {
        detail: Box::new(e),
    })?;
    let underlying = yamls.swap_remove(0);
    let value = YamlValue::try_from(underlying).map_err(Conversion)?;
    let a = A::try_from(value).map_err(Conversion)?;
    Ok(a)
}

#[derive(Debug)]
pub enum YamlLoaderError {
    CannotScanYaml { detail: Box<dyn Debug + Send> },
    Conversion(YamlError),
}

impl From<YamlLoaderError> for Error {
    fn from(this: YamlLoaderError) -> Self {
        Self::YamlLoader(this)
    }
}
