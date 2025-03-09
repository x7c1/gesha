use crate::v3_0::components::schemas::type_header_shape::HeaderBody;
use crate::v3_0::components::schemas::{AllOfItemShapes, FieldShape, OneOfItemShapes, Optionality};
use gesha_core::conversions::Result;
use openapi_types::v3_0::{EnumValues, RequiredSchemaFields, SchemaObject};

#[derive(Clone, Debug)]
pub struct InlineSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub fields: Vec<FieldShape>,
    pub required: Option<RequiredSchemaFields>,
    pub all_of: AllOfItemShapes,
    pub one_of: OneOfItemShapes,
    pub enum_values: Option<EnumValues>,
    pub optionality: Optionality,
}

impl InlineSchema {
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        let all_of = if let Some(all_of) = object.all_of.clone() {
            AllOfItemShapes::from_schema_cases(all_of).to_result()?
        } else {
            AllOfItemShapes::default()
        };
        let one_of = if let Some(one_of) = object.one_of.clone() {
            OneOfItemShapes::from_schema_cases(one_of).to_result()?
        } else {
            OneOfItemShapes::default()
        };
        Ok(Self {
            title: object.title.clone(),
            description: object.description.clone(),
            required: object.required.clone(),
            enum_values: object.enum_values.clone(),
            fields: FieldShape::from_object(object).to_result()?,
            all_of,
            one_of,
            optionality,
        })
    }
    pub fn generate_header_body(&self) -> HeaderBody {
        HeaderBody {
            title: self.title.clone(),
            description: self.description.clone(),
            nullable: Some(self.optionality.is_nullable),
        }
    }
}
