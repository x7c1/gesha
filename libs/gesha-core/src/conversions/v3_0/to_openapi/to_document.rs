use crate::conversions::v3_0::to_openapi::to_paths_object::to_paths_object;
use crate::conversions::Error::IncompatibleVersion;
use crate::conversions::{Result, ToOpenApi};
use crate::yaml::YamlMap;
use openapi_types::v3_0::{Document, InfoObject};

impl ToOpenApi for Document {
    /// return Error::IncompatibleVersion if not supported version.
    fn apply(mut map: YamlMap) -> Result<Self> {
        let components = map
            .remove_if_exists("components")?
            .map(ToOpenApi::apply)
            .transpose()?;

        let document = Document {
            openapi: to_openapi_version(map.remove("openapi")?)?,
            info: to_info(map.remove("info")?)?,
            paths: to_paths_object(map.remove("paths")?)?,
            components,
        };
        Ok(document)
    }
}

fn to_openapi_version(s: String) -> Result<String> {
    if !s.starts_with("3.0.") {
        return Err(IncompatibleVersion);
    }
    Ok(s)
}

fn to_info(mut map: YamlMap) -> Result<InfoObject> {
    let info = InfoObject {
        title: map.remove("title")?,
    };
    Ok(info)
}
