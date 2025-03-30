use crate::Output;
use crate::v3_0::{ComponentsObject, InfoObject, OpenApiVersion, PathsObject, YamlExtractor};
use crate::yaml::{ToOpenApi, YamlMap};

/// OpenAPI Object
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#openapi-object
#[derive(Debug)]
pub struct Document {
    /// > REQUIRED. This string MUST be the version number of the OpenAPI Specification that the OpenAPI Document uses.
    pub openapi: OpenApiVersion,

    /// > REQUIRED. Provides metadata about the API.
    pub info: InfoObject,

    /// > REQUIRED. The available paths and operations for the API.
    pub paths: PathsObject,

    pub components: Option<ComponentsObject>,
}

impl ToOpenApi for Document {
    fn apply(mut map: YamlMap) -> Output<Self> {
        let (openapi, errors_of_openapi) = map
            .extract_with_default("openapi", OpenApiVersion::from_string)
            .into_tuple();

        let (info, errors_of_info) = map
            .extract_with_default("info", InfoObject::from_yaml_map)
            .into_tuple();

        let (paths, errors_of_paths) = map
            .extract_with_default("paths", PathsObject::from_yaml_map)
            .into_tuple();

        let (components, errors_of_components) = map
            .extract_if_exists("components", ToOpenApi::apply)
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
