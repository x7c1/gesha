use crate::{DataType, Definition, TypeHeader};

#[derive(Clone, Debug, PartialEq)]
pub struct NewTypeDef {
    pub header: TypeHeader,
    pub data_type: DataType,
    _hide_default_constructor: bool,
}

impl NewTypeDef {
    pub fn new(header: TypeHeader, data_type: DataType) -> Self {
        Self {
            header,
            data_type,
            _hide_default_constructor: true,
        }
    }
    pub fn symbol_name(&self) -> &str {
        self.header.name.as_ref()
    }
}

impl From<NewTypeDef> for Definition {
    fn from(x: NewTypeDef) -> Self {
        Self::NewTypeDef(x)
    }
}
