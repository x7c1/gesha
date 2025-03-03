use crate::v3_0::components::schemas::type_header_shape::HeaderParts;
use crate::v3_0::components::schemas::{
    AllOfItemShape, AllOfShape, InlineShape, Optionality, RefShape, TypeHeaderShape,
};
use gesha_core::conversions::Result;
use openapi_types::v3_0::{SchemaCase, SchemaObject};

#[derive(Clone, Debug)]
pub struct InlineAllOfShape {
    object: SchemaObject,
    pub optionality: Optionality,
}

impl InlineAllOfShape {
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        Ok(Self {
            object,
            optionality,
        })
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
            nullable: self.object.nullable,
        }
    }
    pub fn pop_if_only_one_ref(&self) -> Result<Option<RefShape>> {
        let Some(all_of) = &self.object.all_of else {
            return Ok(None);
        };
        let ref_object = match all_of.as_slice() {
            [SchemaCase::Reference(object)] => object,
            _ => return Ok(None),
        };
        let required = self.optionality.is_required;

        // TODO: Pass self.optionality.is_nullable to RefShape
        let shape = RefShape::new(ref_object.clone(), required)?;
        Ok(Some(shape))
    }
}

impl From<InlineAllOfShape> for InlineShape {
    fn from(value: InlineAllOfShape) -> Self {
        Self::AllOf(value)
    }
}
