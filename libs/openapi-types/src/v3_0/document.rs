use crate::Unsupported::IncompatibleVersion;
use crate::core::OutputOptionOps;
use crate::v3_0::{ComponentsObject, PathsObject, YamlExtractor};
use crate::yaml::{ToOpenApi, YamlMap};
use crate::{Output, Result, with_key};

/// OpenAPI Document
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schema
#[derive(Debug)]
pub struct Document {
    pub openapi: String,
    pub info: InfoObject,
    pub paths: PathsObject,
    pub components: Option<ComponentsObject>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#infoObject
#[derive(Debug)]
pub struct InfoObject {
    pub title: String,
}

impl ToOpenApi for Document {
    /// return Error::IncompatibleVersion if not supported version.
    fn apply(mut map: YamlMap) -> Output<Self> {
        let (components, errors_of_components) = map
            .flat_extract_if_exists("components", ToOpenApi::apply)
            .into_tuple();

        let (paths, errors_of_paths) = to_paths(&mut map).into_tuple();
        let (openapi, errors_of_openapi) = to_openapi(&mut map).into_tuple();
        let (info, errors_of_info) = to_info(&mut map).into_tuple();

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

fn to_info(map: &mut YamlMap) -> Output<InfoObject> {
    let (mut map, errors_of_info) = map
        .extract::<YamlMap>("info")
        .maybe()
        .map(|maybe| maybe.unwrap_or_default())
        .into_tuple();

    let (title, errors_of_title) = map
        .extract::<String>("title")
        .maybe()
        .map(|maybe| maybe.unwrap_or_default())
        .into_tuple();

    let info = InfoObject { title };

    Output::ok(info)
        .append(errors_of_info)
        .append(errors_of_title)
        .bind_errors(with_key("info"))
}

fn to_paths(map: &mut YamlMap) -> Output<PathsObject> {
    let (map, errors1) = map
        .extract::<YamlMap>("paths")
        .maybe()
        .map(|maybe| maybe.unwrap_or_default())
        .into_tuple();

    let (paths, errors2) = PathsObject::from_yaml_map(map).into_tuple();

    Output::ok(paths)
        .append(errors1)
        .append(errors2)
        .bind_errors(with_key("paths"))
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
