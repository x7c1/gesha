use crate::v3_0::{MediaTypeKey, MediaTypeObject};
use indexmap::IndexMap;

type InnerMap = IndexMap<MediaTypeKey, MediaTypeObject>;

#[derive(Clone, Debug)]
pub struct RequestBodyContent(InnerMap);

impl RequestBodyContent {
    pub fn new(map: InnerMap) -> Self {
        Self(map)
    }
}

impl IntoIterator for RequestBodyContent {
    type Item = (MediaTypeKey, MediaTypeObject);
    type IntoIter = <InnerMap as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
