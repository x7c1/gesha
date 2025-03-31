use crate::v3_0::{ResponsesObject, YamlMapExt};
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
        let (responses, errors_of_responses) = map
            .extract("responses", ResponsesObject::from_yaml_map)?
            .into_tuple();

        let object = OperationObject { responses };
        Ok(Output::new(object, errors_of_responses))
    }
}
