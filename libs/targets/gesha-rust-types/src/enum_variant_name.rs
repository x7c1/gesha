use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct EnumVariantName(TypeIdentifier);

impl EnumVariantName {
    pub fn new<A: AsRef<str>>(x: A) -> Self {
        let identifier = TypeIdentifier::generate(x);
        Self(identifier)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for EnumVariantName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<EnumVariantName> for String {
    fn from(this: EnumVariantName) -> Self {
        String::from(this.0)
    }
}

use crate::TypeIdentifier;
