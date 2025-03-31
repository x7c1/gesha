use crate::v3_0::{MediaTypeKey, MediaTypeObject};
use crate::{Output, Result, by_key};
use gesha_collections::yaml::YamlMap;
use indexmap::IndexMap;

type InnerMap = IndexMap<MediaTypeKey, MediaTypeObject>;
type InnerEntry = (MediaTypeKey, MediaTypeObject);

#[derive(Clone, Debug)]
pub struct RequestBodyContent(InnerMap);

impl RequestBodyContent {
    pub fn new(map: InnerMap) -> Self {
        Self(map)
    }

    pub fn with_name(kv: (String, YamlMap)) -> Result<Output<(MediaTypeKey, MediaTypeObject)>> {
        let (name, map) = kv;
        let key = MediaTypeKey::new(name);
        let output = MediaTypeObject::from_yaml_map(map)
            .map_err(by_key(key.clone()))?
            .map(|object| (key, object));

        Ok(output)
    }
}

impl FromIterator<InnerEntry> for RequestBodyContent {
    fn from_iter<T: IntoIterator<Item = InnerEntry>>(iter: T) -> Self {
        let map = iter.into_iter().collect();
        Self::new(map)
    }
}

impl IntoIterator for RequestBodyContent {
    type Item = (MediaTypeKey, MediaTypeObject);
    type IntoIter = <InnerMap as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
