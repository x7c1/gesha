use crate::targets::rust_type::DataType;
use heck::ToUpperCamelCase;
use std::fmt::{Display, Formatter};

/// rf. https://doc.rust-lang.org/reference/items/enumerations.html
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum EnumCase {
    Unit,
    Tuple(Vec<DataType>),
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
