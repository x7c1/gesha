use crate::v3_0::yaml_extractor::collect;
use crate::v3_0::{RequestBodyContent, YamlExtractor};
use crate::yaml::YamlMap;
use crate::{Error, Output, Result};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#request-body-object
#[derive(Clone, Debug)]
pub struct RequestBodyObject {
    pub description: Option<String>,

    /// > REQUIRED. The content of the request body.
    pub content: RequestBodyContent,

    /// > Determines if the request body is required in the request. Defaults to false.
    pub required: bool,
}

impl RequestBodyObject {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<RequestBodyObject> {
        let (content, errors_of_content) = map
            .transform("content", collect(RequestBodyContent::with_name))?
            .into_tuple();

        let (required, errors_of_required) = map
            .extract_if_exists("required")
            .map(|maybe| maybe.unwrap_or(false))
            .into_tuple();

        let (description, errors_of_description) =
            map.extract_if_exists("description").into_tuple();

        let object = RequestBodyObject {
            description,
            content,
            required,
        };
        let output = Output::ok(object)
            .append(errors_of_content)
            .append(errors_of_required)
            .append(errors_of_description);

        output.to_result().map_err(Error::multiple)
    }
}
