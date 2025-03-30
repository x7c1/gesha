use crate::Unsupported::IncompatibleVersion;
use crate::core::OutputOptionOps;
use crate::v3_0::{ComponentsObject, InfoObject, PathsObject, YamlExtractor};
use crate::yaml::{ToOpenApi, YamlMap};
use crate::{Output, Result, with_key};

/// OpenAPI Object
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#openapi-object
#[derive(Debug)]
pub struct Document {
    /// > REQUIRED. This string MUST be the version number of the OpenAPI Specification that the OpenAPI Document uses.
    pub openapi: String,

    /// > REQUIRED. Provides metadata about the API.
    pub info: InfoObject,

    /// > REQUIRED. The available paths and operations for the API.
    pub paths: PathsObject,

    pub components: Option<ComponentsObject>,
}

impl ToOpenApi for Document {
    /// return Error::IncompatibleVersion if not supported version.
    fn apply(mut map: YamlMap) -> Output<Self> {
        // TODO:
        let (openapi, errors_of_openapi) = to_openapi(&mut map).into_tuple();

        let (info, errors_of_info) = map
            .extract_or_by_default("info", InfoObject::from_yaml_map)
            .into_tuple();

        let (paths, errors_of_paths) = map
            .extract_or_by_default("paths", PathsObject::from_yaml_map)
            .into_tuple();

        let (components, errors_of_components) = map
            .flat_extract_if_exists("components", ToOpenApi::apply)
            .into_tuple();

        let document = Document {
            openapi,
            info,
            paths,
            components,
        };

        Output::ok(document)
            .append(errors_of_openapi)
            .append(errors_of_info)
            .append(errors_of_paths)
            .append(errors_of_components)
    }
}

fn to_openapi_version(version: String) -> Result<String> {
    if !version.starts_with("3.0.") {
        Err(IncompatibleVersion {
            version: version.clone(),
        })?;
    }
    Ok(version)
}

fn to_openapi(map: &mut YamlMap) -> Output<String> {
    let output = map
        .extract::<String>("openapi")
        .maybe()
        .map(|maybe| maybe.unwrap_or_default());

    output
        .map(to_openapi_version)
        .maybe()
        .map(|maybe| maybe.unwrap_or_default())
        .bind_errors(with_key("openapi"))
}
