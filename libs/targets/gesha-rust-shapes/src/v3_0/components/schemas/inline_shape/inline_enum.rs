use crate::v3_0::components::schemas::{EnumShape, InlineShape, TypeHeaderShape};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub struct InlineEnumShape {}

impl InlineEnumShape {
    pub fn new(_object: SchemaObject) -> Result<Self> {
        todo!()
    }
    pub fn expand_with(&self, _shape: TypeHeaderShape) -> Result<EnumShape> {
        todo!()
    }
}

impl From<InlineEnumShape> for InlineShape {
    fn from(value: InlineEnumShape) -> Self {
        InlineShape::Enum(value)
    }
}
