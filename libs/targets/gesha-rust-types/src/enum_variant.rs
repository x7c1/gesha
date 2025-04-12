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
    pub fn unit(
        name: EnumVariantName,
        constant: EnumConstant,
        attributes: Vec<EnumVariantAttribute>,
    ) -> Self {
        EnumVariant {
            name,
            attributes,
            case: EnumCase::Unit(constant),
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
    Unit(EnumConstant),
    Tuple(Vec<DataType>),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum EnumConstant {
    Bool(bool),
    I32(i32),
    I64(i64),
    Null,
    Str(String),
}

impl EnumConstant {
    pub fn type_name(&self) -> &str {
        match self {
            EnumConstant::Bool(_) => "bool",
            EnumConstant::I32(_) => "i32",
            EnumConstant::I64(_) => "i64",
            EnumConstant::Null => "null",
            EnumConstant::Str(_) => "str",
        }
    }
}

impl Display for EnumConstant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnumConstant::Bool(x) => Display::fmt(x, f),
            EnumConstant::I32(x) => Display::fmt(x, f),
            EnumConstant::I64(x) => Display::fmt(x, f),
            EnumConstant::Null => Display::fmt("null", f),
            EnumConstant::Str(x) => Display::fmt(&format!(r#""{x}""#), f),
        }
    }
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
