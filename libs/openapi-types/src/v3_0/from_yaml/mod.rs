use crate::Error::IncompatibleVersion;
use crate::core::OutputOptionOps;
use crate::v3_0::{Document, InfoObject, PathsObject};
use crate::yaml::{ToOpenApi, YamlMap};
use crate::{Output, Result, with_key};

impl ToOpenApi for Document {
    /// return Error::IncompatibleVersion if not supported version.
    fn apply(mut map: YamlMap) -> Result<Output<Self>> {
        let (components, components_errors) = map
            .remove_if_exists("components")?
            .map(ToOpenApi::apply)
            .transpose()?
            .maybe()
            .bind_errors(with_key("components"))
            .into_tuple();

        let (paths, paths_errors) = {
            let map = map.remove("paths")?;
            PathsObject::from_yaml_map(map)
                .bind_errors(with_key("paths"))
                .into_tuple()
        };

        let document = Document {
            openapi: to_openapi_version(map.remove("openapi")?)?,
            info: to_info(map.remove("info")?)?,
            paths,
            components,
        };
        let output = Output::new(document, components_errors).append(paths_errors);
        Ok(output)
    }
}

fn to_openapi_version(version: String) -> Result<String> {
    if !version.starts_with("3.0.") {
        return Err(IncompatibleVersion { version });
    }
    Ok(version)
}

fn to_info(mut map: YamlMap) -> Result<InfoObject> {
    let info = InfoObject {
        title: map.remove("title")?,
    };
    Ok(info)
}
