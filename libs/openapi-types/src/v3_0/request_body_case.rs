use crate::Unsupported::Unimplemented;
use crate::v3_0::{ComponentName, ReferenceObject, RequestBodyObject, YamlMapExt};
use crate::yaml::YamlMap;
use crate::{Result, by_key};

/// Request Body Object | Reference Object
#[derive(Clone, Debug)]
pub enum RequestBodyCase {
    RequestBody(Box<RequestBodyObject>),
    Reference(ReferenceObject<RequestBodyObject>),
}

impl RequestBodyCase {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<RequestBodyCase> {
        map.error_if_exists("$ref", |_: String| Unimplemented {
            message: "$ref in requestBody is not supported.".into(),
        })?;
        let object = RequestBodyObject::from_yaml_map(map)?;
        let case = RequestBodyCase::RequestBody(Box::new(object));
        Ok(case)
    }

    pub fn with_name(kv: (String, YamlMap)) -> Result<(ComponentName, RequestBodyCase)> {
        let (name, map) = kv;
        let pair = (
            ComponentName::new(&name),
            Self::from_yaml_map(map).map_err(by_key(name))?,
        );
        Ok(pair)
    }
}
