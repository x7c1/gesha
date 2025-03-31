use crate::v3_0::{SchemaCase, YamlMapExt};
use crate::{Output, Result};
use gesha_collections::yaml::YamlMap;

/// > The key is a media type or media type range and the value describes it.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct MediaTypeKey(String);

impl MediaTypeKey {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<MediaTypeKey> for String {
    fn from(this: MediaTypeKey) -> Self {
        this.0
    }
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#media-type-object
#[derive(Clone, Debug)]
pub struct MediaTypeObject {
    pub schema: Option<SchemaCase>,
}

impl MediaTypeObject {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<Output<Self>> {
        let output = map.extract_if_exists("schema", SchemaCase::from_yaml_map);
        Ok(output.map(|schema| Self { schema }))
    }
}
