use crate::Error::Yaml;
use crate::yaml::{YamlError, YamlValue};
use crate::{Error, Result};
use yaml_rust::YamlLoader;

pub fn load_from_str<A>(contents: &str) -> Result<A>
where
    A: TryFrom<YamlValue, Error = YamlError>,
{
    let mut yamls = YamlLoader::load_from_str(contents).map_err(|e| Error::CannotScanYaml {
        detail: Box::new(e),
    })?;
    let underlying = yamls.swap_remove(0);
    let value = YamlValue::try_from(underlying).map_err(Yaml)?;
    A::try_from(value).map_err(Yaml)
}
