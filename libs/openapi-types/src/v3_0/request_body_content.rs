use crate::v3_0::{MediaTypeKey, MediaTypeObject};
use indexmap::IndexMap;

#[derive(Clone, Debug)]
pub struct RequestBodyContent(IndexMap<MediaTypeKey, MediaTypeObject>);

impl RequestBodyContent {
    pub fn new(map: IndexMap<MediaTypeKey, MediaTypeObject>) -> Self {
        Self(map)
    }
}
