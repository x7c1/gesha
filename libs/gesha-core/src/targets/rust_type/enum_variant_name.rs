use heck::ToUpperCamelCase;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnumVariantName(String);

impl EnumVariantName {
    pub fn new<A: AsRef<str>>(x: A) -> Self {
        // TODO: replace x with Rust compatible chars if illegal chars are included
        Self(x.as_ref().to_upper_camel_case())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for EnumVariantName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<EnumVariantName> for String {
    fn from(this: EnumVariantName) -> Self {
        this.0
    }
}

impl Borrow<str> for EnumVariantName {
    fn borrow(&self) -> &str {
        &self.0
    }
}
