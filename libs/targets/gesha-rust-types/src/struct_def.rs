use crate::{Definition, StructField, TypeHeader};

#[derive(Clone, Debug, PartialEq)]
pub struct StructDef {
    pub header: TypeHeader,
    pub fields: Vec<StructField>,
    _hide_default_constructor: bool,
}

impl StructDef {
    pub fn new(header: TypeHeader, fields: Vec<StructField>) -> Self {
        Self {
            header,
            fields,
            _hide_default_constructor: true,
        }
    }
}

impl From<StructDef> for Definition {
    fn from(x: StructDef) -> Self {
        Self::StructDef(x)
    }
}
