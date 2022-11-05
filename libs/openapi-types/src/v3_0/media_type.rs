use crate::v3_0::SchemaCase;

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

/// rf. https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#media-type-object
#[derive(Clone, Debug)]
pub struct MediaTypeObject {
    pub schema: SchemaCase,
}
