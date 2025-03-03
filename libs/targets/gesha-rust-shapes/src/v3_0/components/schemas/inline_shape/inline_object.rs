use crate::v3_0::components::schemas::inline_shape::InlineSchemaShape;
use crate::v3_0::components::schemas::type_header_shape::HeaderParts;
use crate::v3_0::components::schemas::{AllOfItemShape, Optionality, RefShape};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub struct InlineObject {
    pub object: InlineSchemaShape,
    pub optionality: Optionality,
}

impl InlineObject {
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        Ok(Self {
            object: InlineSchemaShape::shape(object)?,
            optionality,
        })
    }
    pub fn generate_header_parts(&self) -> HeaderParts {
        HeaderParts {
            title: self.object.title.clone(),
            description: self.object.description.clone(),
            nullable: self.object.nullable,
        }
    }
    pub fn pop_all_of_if_single_ref(&self) -> Result<Option<RefShape>> {
        let ref_shape = match self.object.all_of.as_slice() {
            [AllOfItemShape::Ref(object)] => object,
            _ => return Ok(None),
        };
        let mut ref_shape = ref_shape.clone();
        ref_shape.is_required = self.optionality.is_required;
        ref_shape.nullable = Some(self.optionality.is_nullable);

        Ok(Some(ref_shape.clone()))
    }
}
