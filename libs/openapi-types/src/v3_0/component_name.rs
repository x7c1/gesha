use std::fmt::{Display, Formatter};

/// > All the fixed fields declared above are objects
/// > that MUST use keys that match the regular expression: ^[a-zA-Z0-9\.\-_]+$.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SchemaFieldName(String);

impl SchemaFieldName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        SchemaFieldName(a.into())
    }
}

impl From<SchemaFieldName> for String {
    fn from(this: SchemaFieldName) -> Self {
        this.0
    }
}

impl AsRef<str> for SchemaFieldName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for SchemaFieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
