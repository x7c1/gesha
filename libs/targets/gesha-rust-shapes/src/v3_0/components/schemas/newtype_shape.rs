use crate::v3_0::components::schemas::{DefinitionShape, TypeHeaderShape, TypeShape};

#[derive(Clone, Debug)]
pub struct NewTypeShape {
    pub header: TypeHeaderShape,
    pub type_shape: TypeShape,
}

impl NewTypeShape {
    pub fn new(header: TypeHeaderShape, type_shape: TypeShape) -> Self {
        Self { header, type_shape }
    }
}

impl From<NewTypeShape> for DefinitionShape {
    fn from(this: NewTypeShape) -> Self {
        Self::NewType(this)
    }
}
