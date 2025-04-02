use crate::{DataType, DocComments, StructFieldName};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct StructField {
    pub name: StructFieldName,
    pub data_type: DataType,
    pub attributes: Vec<StructFieldAttribute>,
    pub doc_comments: Option<DocComments>,
    _hide_default_constructor: bool,
}

impl StructField {
    pub fn new(
        name: StructFieldName,
        data_type: DataType,
        attributes: Vec<StructFieldAttribute>,
        doc_comments: Option<DocComments>,
    ) -> Self {
        Self {
            name,
            data_type,
            attributes,
            doc_comments,
            _hide_default_constructor: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
