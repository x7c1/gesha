use crate::v3_0::components::schemas::{AllOfItemShape, FieldShape};
use gesha_core::conversions::Result;
use openapi_types::v3_0::{RequiredSchemaFields, SchemaObject};

#[derive(Clone, Debug)]
pub struct InlineSchemaShape {
    pub title: Option<String>,
    pub description: Option<String>,
    pub fields: Vec<FieldShape>,
    pub nullable: Option<bool>,
    pub required: Option<RequiredSchemaFields>,
    pub all_of: Vec<AllOfItemShape>,
}

impl InlineSchemaShape {
    pub fn shape(object: SchemaObject) -> Result<Self> {
        let all_of = if let Some(all_of) = object.all_of.clone() {
            AllOfItemShape::from_schema_cases(all_of).to_result()?
        } else {
            vec![]
        };
        Ok(Self {
            title: object.title.clone(),
            description: object.description.clone(),
            nullable: object.nullable,
            required: object.required.clone(),
            fields: FieldShape::from_object(object).to_result()?,
            all_of,
        })
    }
}
