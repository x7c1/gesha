use crate::{DataType, EnumVariantName};
use std::fmt::{Display, Formatter};

/// rf. https://doc.rust-lang.org/reference/items/enumerations.html
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct EnumVariant {
    pub name: EnumVariantName,
    pub attributes: Vec<EnumVariantAttribute>,
    pub case: EnumCase,
    _hide_default_constructor: bool,
}

impl EnumVariant {
    pub fn unit(name: EnumVariantName, attributes: Vec<EnumVariantAttribute>) -> Self {
        EnumVariant {
            name,
            attributes,
            case: EnumCase::Unit,
            _hide_default_constructor: true,
        }
    }
    pub fn tuple(
        name: EnumVariantName,
        types: Vec<DataType>,
        attributes: Vec<EnumVariantAttribute>,
    ) -> Self {
        EnumVariant {
            name,
            attributes,
            case: EnumCase::Tuple(types),
            _hide_default_constructor: true,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum EnumCase {
    Unit,
    Tuple(Vec<DataType>),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct EnumVariantAttribute(String);

impl EnumVariantAttribute {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
}

impl Display for EnumVariantAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
