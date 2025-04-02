use crate::TypeIdentifier;
use gesha_core::conversions::Result;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct EnumVariantName(TypeIdentifier);

impl EnumVariantName {
    pub fn new<A: AsRef<str>>(x: A) -> Result<Self> {
        let identifier = TypeIdentifier::parse(x)?;
        Ok(Self(identifier))
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
