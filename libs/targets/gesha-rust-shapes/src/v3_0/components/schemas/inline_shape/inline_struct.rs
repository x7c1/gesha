use crate::v3_0::components::schemas::type_header_shape::HeaderParts;
use crate::v3_0::components::schemas::{FieldShape, Optionality, StructShape, TypeHeaderShape};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub struct InlineStructShape {
    pub object: SchemaObject,
    pub optionality: Optionality,
}

impl InlineStructShape {
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        Ok(Self {
            object,
            optionality,
        })
    }
    pub fn expand_with(self, header: TypeHeaderShape) -> Result<StructShape> {
        let fields = FieldShape::from_object(self.object).to_result()?;
        Ok(StructShape { header, fields })
    }
    pub fn generate_header_parts(&self) -> HeaderParts {
        HeaderParts {
            title: self.object.title.clone(),
            description: self.object.description.clone(),
            nullable: self.object.nullable.clone(),
        }
    }
}
