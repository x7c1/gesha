mod to_components_object;
mod to_paths_object;
use to_paths_object::to_paths_object;

mod to_request_body;
use to_request_body::to_request_body_pair;

mod to_schema_case;
use to_schema_case::{to_schema_case, to_schema_pair};

use crate::v3_0::{Document, InfoObject};
use crate::yaml::{ToOpenApi, YamlMap};
use crate::Error::IncompatibleVersion;
use crate::{Error, OptionOutputOps, Output, Result};

impl ToOpenApi for Document {
    /// return Error::IncompatibleVersion if not supported version.
    fn apply(mut map: YamlMap) -> Result<Output<Self>> {
        let (components, errors1) = map
            .remove_if_exists("components")?
            .map(ToOpenApi::apply)
            .transpose()?
            .maybe()
            .map_errors(Error::with_key("components"))
            .to_tuple();

        let (paths, errors2) = {
            let map = map.remove("paths")?;
            to_paths_object(map)?
                .map_errors(Error::with_key("paths"))
                .to_tuple()
        };

        let document = Document {
            openapi: to_openapi_version(map.remove("openapi")?)?,
            info: to_info(map.remove("info")?)?,
            paths,
            components,
        };
        Ok(Output::new(document, errors1).append(errors2))
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
