use crate::conversions::v3_0::to_rust_type::components::from_schemas::{
    DefinitionShape, FieldShape, TypeHeaderShape,
};

#[derive(Clone, Debug)]
pub struct StructShape {
    pub header: TypeHeaderShape,
    pub fields: Vec<FieldShape>,
}

impl From<StructShape> for DefinitionShape {
    fn from(this: StructShape) -> Self {
        Self::Struct(this)
    }
}
