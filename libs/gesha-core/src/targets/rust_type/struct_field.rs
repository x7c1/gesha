use crate::targets::rust_type::{DataType, StructFieldName};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct StructField {
    pub name: StructFieldName,
    pub data_type: DataType,
    pub attributes: Vec<StructFieldAttribute>,
    _hide_default_constructor: bool,
}

impl StructField {
    pub fn new(
        name: StructFieldName,
        data_type: DataType,
        attributes: Vec<StructFieldAttribute>,
    ) -> Self {
        Self {
            name,
            data_type,
            attributes,
            _hide_default_constructor: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StructFieldAttribute(String);

impl StructFieldAttribute {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
}

impl Display for StructFieldAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
