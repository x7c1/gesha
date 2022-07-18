use heck::ToUpperCamelCase;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub name: EnumVariantName,
    pub attributes: Vec<EnumVariantAttribute>,
    _hide_default_constructor: bool,
}

impl EnumVariant {
    pub fn new(name: EnumVariantName, attributes: Vec<EnumVariantAttribute>) -> Self {
        EnumVariant {
            name,
            attributes,
            _hide_default_constructor: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumVariantName(String);

impl EnumVariantName {
    pub fn new(x: &str) -> Self {
        // TODO: replace x with Rust compatible chars if illegal chars are included
        Self(x.to_upper_camel_case())
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
