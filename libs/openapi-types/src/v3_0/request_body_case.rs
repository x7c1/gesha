use crate::Unsupported::Unimplemented;
use crate::v3_0::{ComponentName, ReferenceObject, RequestBodyObject};
use crate::{Error, Result};
use gesha_collections::tracking::TrackingKeyAppendable;
use gesha_collections::yaml::{YamlMap, YamlMapExt};

/// Request Body Object | Reference Object
#[derive(Clone, Debug)]
pub enum RequestBodyCase {
    RequestBody(Box<RequestBodyObject>),
    Reference(ReferenceObject<RequestBodyObject>),
}

impl RequestBodyCase {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<RequestBodyCase> {
        map.error_if_exists("$ref", |_: String| {
            Error::Unsupported(Unimplemented {
                message: "$ref in requestBody is not supported.".into(),
            })
        })?;
        let object = RequestBodyObject::from_yaml_map(map)?;
        let case = RequestBodyCase::RequestBody(Box::new(object));
        Ok(case)
    }

    pub fn with_name(kv: (String, YamlMap)) -> Result<(ComponentName, RequestBodyCase)> {
        let (name, map) = kv;
        let pair = (
            ComponentName::new(&name),
            Self::from_yaml_map(map).with_key(name)?,
        );
        Ok(pair)
    }
}
