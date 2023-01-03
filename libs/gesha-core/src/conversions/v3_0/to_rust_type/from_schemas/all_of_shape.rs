use crate::conversions::v3_0::to_rust_type::from_schemas::{
    AllOfItemShape, DefinitionShape, TypeHeaderShape,
};

#[derive(Clone, Debug)]
pub struct AllOfShape {
    pub header: TypeHeaderShape,
    pub items: Vec<AllOfItemShape>,
}

impl From<AllOfShape> for DefinitionShape {
    fn from(this: AllOfShape) -> Self {
        Self::AllOf(this)
    }
}
