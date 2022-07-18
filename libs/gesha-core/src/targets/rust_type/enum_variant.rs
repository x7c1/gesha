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
        // TODO: replace original with Rust compatible chars if illegal chars are included.
        EnumVariant {
            name,
            attributes,
            _hide_default_constructor: true,
        }
    }
}
#[derive(Clone, Debug)]
pub struct EnumVariantName {
    pub compatible: String,
    pub original: String,
    _hide_default_constructor: bool,
}

impl EnumVariantName {
    pub fn new(original: String) -> Self {
        Self {
            compatible: original.to_upper_camel_case(),
            original,
            _hide_default_constructor: true,
        }
    }
    pub(crate) fn find_to_rename(&self) -> Option<&str> {
        if self.compatible == self.original {
            None
        } else {
            Some(&self.original)
        }
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
