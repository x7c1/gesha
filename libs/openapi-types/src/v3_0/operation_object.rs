use crate::error::by_key;
use crate::v3_0::{ResponsesObject, YamlExtractor};
use crate::yaml::YamlMap;
use crate::{Output, Result};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#operation-object
#[derive(Debug)]
pub struct OperationObject {
    /// > REQUIRED. The list of possible responses as they are returned from executing this operation.
    pub responses: ResponsesObject,
}

impl OperationObject {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<Output<Self>> {
        let responses = map.extract("responses")?;
        let (responses, errors) = ResponsesObject::from_yaml_map(responses)
            .map_err(by_key("responses"))?
            .into_tuple();

        let object = OperationObject { responses };
        Ok(Output::new(object, errors))
    }
}
