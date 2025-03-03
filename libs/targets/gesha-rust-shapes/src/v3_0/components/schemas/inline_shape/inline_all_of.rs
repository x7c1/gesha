use crate::v3_0::components::schemas::inline_shape::InlineSchemaShape;
use crate::v3_0::components::schemas::type_header_shape::HeaderParts;
use crate::v3_0::components::schemas::{
    AllOfItemShape, AllOfShape, InlineShape, Optionality, RefShape, TypeHeaderShape,
};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub struct InlineAllOfShape {
    object: InlineSchemaShape,
    pub optionality: Optionality,
}

impl InlineAllOfShape {
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        Ok(Self {
            object: InlineSchemaShape::shape(object)?,
            optionality,
        })
    }
    pub fn expand_with(self, header: TypeHeaderShape) -> Result<AllOfShape> {
        let shape = AllOfShape {
            header,
            items: self.object.all_of,
            required: self.object.required,
        };
        Ok(shape)
    }
    pub fn generate_header_parts(&self) -> HeaderParts {
        HeaderParts {
            title: self.object.title.clone(),
            description: self.object.description.clone(),
            nullable: self.object.nullable,
        }
    }
    pub fn pop_if_only_one_ref(&self) -> Result<Option<RefShape>> {
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

impl From<InlineAllOfShape> for InlineShape {
    fn from(value: InlineAllOfShape) -> Self {
        Self::AllOf(value)
    }
}
