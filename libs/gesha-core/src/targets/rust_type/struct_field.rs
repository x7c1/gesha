use crate::targets::rust_type::{DataType, StructFieldName};

#[derive(Clone, Debug)]
pub struct StructField {
    pub name: StructFieldName,
    pub data_type: DataType,
    _hide_default_constructor: bool,
}

impl StructField {
    pub fn new(name: StructFieldName, data_type: DataType) -> Self {
        // TODO: add field attributes
        Self {
            name,
            data_type,
            _hide_default_constructor: true,
        }
    }
}
