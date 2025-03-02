use crate::v3_0::components::schemas::type_header_shape::HeaderParts;
use crate::v3_0::components::schemas::{AllOfItemShape, AllOfShape, TypeHeaderShape};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub struct InlineAllOfShape {
    object: SchemaObject,
}

impl InlineAllOfShape {
    pub fn new(object: SchemaObject) -> Result<Self> {
        Ok(Self { object })
    }
    pub fn expand_with(self, header: TypeHeaderShape) -> Result<AllOfShape> {
        // TODO: remove unwrap
        let cases = self.object.all_of.unwrap();
        let shape = AllOfShape {
            header,
            items: AllOfItemShape::from_schema_cases(cases).to_result()?,
            required: self.object.required,
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
