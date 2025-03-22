use crate::error::with_key;
use crate::v3_0::{ResponsesObject, YamlExtractor};
use crate::yaml::YamlMap;
use crate::{Output, Result};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#operationObject
#[derive(Debug)]
pub struct OperationObject {
    pub responses: ResponsesObject,
}

impl OperationObject {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<Output<Self>> {
        let responses = map.extract("responses")?;
        let (responses, errors) = ResponsesObject::from_yaml_map(responses)
            .bind_errors(with_key("responses"))
            .into_tuple();

        let object = OperationObject { responses };
        Ok(Output::new(object, errors))
    }
}
