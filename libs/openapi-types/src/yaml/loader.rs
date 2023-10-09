use crate::yaml::YamlValue;
use crate::{Error, Result};
use yaml_rust::YamlLoader;

pub fn load_from_str<A>(contents: &str) -> Result<A>
where
    A: TryFrom<YamlValue, Error = Error>,
{
    let mut yamls = YamlLoader::load_from_str(contents).map_err(|e| Error::CannotScanYaml {
        detail: format!("{:?}", e),
    })?;
    let underlying = yamls.swap_remove(0);
    let value: YamlValue = underlying.try_into()?;
    value.try_into()
}
