use crate::core::OutputOptionOps;
use crate::error::by_key;
use crate::v3_0::{OperationObject, PathFieldName, YamlExtractor};
use crate::yaml::YamlMap;
use crate::{Error, Output, Result};

#[derive(Debug)]
pub struct PathItemObject {
    pub get: Option<OperationObject>,
    pub post: Option<OperationObject>,
}

impl PathItemObject {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<Self> {
        let (get, get_errors) = map
            .try_extract_if_exists("get", OperationObject::from_yaml_map)
            .map(|x| x.maybe())
            .flatten()
            .into_tuple();

        let (post, post_errors) = map
            .try_extract_if_exists("post", OperationObject::from_yaml_map)
            .map(|x| x.maybe())
            .flatten()
            .into_tuple();

        let object = PathItemObject { get, post };
        let output = Output::ok(object).append(get_errors).append(post_errors);
        output.to_result().map_err(Error::multiple)
    }

    pub fn with_name(kv: (String, YamlMap)) -> Result<(PathFieldName, PathItemObject)> {
        let (field, map) = kv;
        let pair = (
            PathFieldName::new(&field)?,
            PathItemObject::from_yaml_map(map).map_err(by_key(field))?,
        );
        Ok(pair)
    }
}
