use heck::ToUpperCamelCase;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct TypeIdentifier(String);

impl TypeIdentifier {
    pub fn new<A: AsRef<str>>(a: A) -> Self {
        // TODO: replace x with Rust compatible chars if illegal chars are included
        let converted = a.as_ref().to_upper_camel_case();
        Self(converted)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for TypeIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<TypeIdentifier> for String {
    fn from(this: TypeIdentifier) -> Self {
        this.0
    }
}
