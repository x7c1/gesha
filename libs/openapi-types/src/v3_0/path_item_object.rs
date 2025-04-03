use crate::v3_0::{OperationObject, PathFieldName};
use crate::{Error, Output, Result};
use gesha_collections::tracking::TrackingKeyAppendable;
use gesha_collections::yaml::{YamlMap, YamlMapExt};

#[derive(Debug)]
pub struct PathItemObject {
    pub get: Option<OperationObject>,
    pub post: Option<OperationObject>,
}

impl PathItemObject {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<Self> {
        let (get, get_errors) = map
            .extractor("get", OperationObject::from_yaml_map)
            .as_optional::<Output<_>>()
            .into_tuple();

        let (post, post_errors) = map
            .extractor("post", OperationObject::from_yaml_map)
            .as_optional::<Output<_>>()
            .into_tuple();

        let object = PathItemObject { get, post };
        let output = Output::ok(object).append(get_errors).append(post_errors);
        output.to_result().map_err(Error::multiple)
    }

    pub fn with_name(kv: (String, YamlMap)) -> Result<(PathFieldName, PathItemObject)> {
        let (field, map) = kv;
        let pair = (
            PathFieldName::new(&field)?,
            PathItemObject::from_yaml_map(map).with_key(field)?,
        );
        Ok(pair)
    }
}
