use crate::v3_0::{ComponentName, ReferenceObject, RequestBodyObject, YamlExtractor};
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
        let case = match map.extract_if_exists::<String>("$ref").to_result()? {
            Some(_reference) => unimplemented!(),
            None => {
                let object = RequestBodyObject::from_yaml_map(map)?;
                RequestBodyCase::RequestBody(Box::new(object))
            }
        };
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
