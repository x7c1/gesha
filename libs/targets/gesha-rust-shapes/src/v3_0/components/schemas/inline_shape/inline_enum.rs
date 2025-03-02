use crate::v3_0::components::schemas::type_header_shape::HeaderParts;
use crate::v3_0::components::schemas::{EnumShape, InlineShape, TypeHeaderShape};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub struct InlineEnumShape {
    object: SchemaObject,
}

impl InlineEnumShape {
    pub fn new(object: SchemaObject) -> Result<Self> {
        Ok(Self { object })
    }
    pub fn expand_with(self, header: TypeHeaderShape) -> Result<EnumShape> {
        // TODO: remove unwrap
        let values = self.object.enum_values.unwrap();
        Ok(EnumShape::new(header, values.clone()))
    }
    pub fn generate_header_parts(&self) -> HeaderParts {
        HeaderParts {
            title: self.object.title.clone(),
            description: self.object.description.clone(),
            nullable: self.object.nullable.clone(),
        }
    }
}

impl From<InlineEnumShape> for InlineShape {
    fn from(value: InlineEnumShape) -> Self {
        InlineShape::Enum(value)
    }
}
