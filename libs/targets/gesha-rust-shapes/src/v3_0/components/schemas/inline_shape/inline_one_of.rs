use crate::v3_0::components::schemas::type_header_shape::HeaderParts;
use crate::v3_0::components::schemas::{OneOfItemShape, OneOfShape, Optionality, TypeHeaderShape};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub struct InlineOneOfShape {
    object: SchemaObject,
    pub optionality: Optionality,
}

impl InlineOneOfShape {
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        Ok(Self {
            object,
            optionality,
        })
    }
    pub fn expand_with(&self, header: TypeHeaderShape) -> Result<OneOfShape> {
        // TODO: remove unwrap
        let cases = self.object.one_of.clone().unwrap();
        let shape = OneOfShape {
            header,
            items: OneOfItemShape::from_schema_cases(cases)?,
        };
        Ok(shape)
    }
    pub fn generate_header_parts(&self) -> HeaderParts {
        HeaderParts {
            title: self.object.title.clone(),
            description: self.object.description.clone(),
            nullable: self.object.nullable.clone(),
        }
    }
}
