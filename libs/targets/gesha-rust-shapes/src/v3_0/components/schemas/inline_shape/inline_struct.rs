use crate::v3_0::components::schemas::inline_shape::InlineSchemaShape;
use crate::v3_0::components::schemas::type_header_shape::HeaderParts;
use crate::v3_0::components::schemas::{InlineShape, Optionality, StructShape, TypeHeaderShape};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub struct InlineStructShape {
    pub object: InlineSchemaShape,
    pub optionality: Optionality,
}

impl InlineStructShape {
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        Ok(Self {
            object: InlineSchemaShape::shape(object)?,
            optionality,
        })
    }
    pub fn expand_with(self, header: TypeHeaderShape) -> Result<StructShape> {
        Ok(StructShape {
            header,
            fields: self.object.fields,
        })
    }
    pub fn generate_header_parts(&self) -> HeaderParts {
        HeaderParts {
            title: self.object.title.clone(),
            description: self.object.description.clone(),
            nullable: self.object.nullable,
        }
    }
}

impl From<InlineStructShape> for InlineShape {
    fn from(value: InlineStructShape) -> Self {
        Self::Struct(value)
    }
}
