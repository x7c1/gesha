use crate::v3_0::components::schemas::type_header_shape::HeaderBody;
use crate::v3_0::components::schemas::{
    AllOfItemShape, FieldShape, OneOfItemShape, Optionality, RefShape,
};
use gesha_core::conversions::Result;
use openapi_types::v3_0::{EnumValues, RequiredSchemaFields, SchemaObject};

#[derive(Clone, Debug)]
pub struct InlineSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub fields: Vec<FieldShape>,
    pub required: Option<RequiredSchemaFields>,
    pub all_of: Vec<AllOfItemShape>,
    pub one_of: Vec<OneOfItemShape>,
    pub enum_values: Option<EnumValues>,
    pub optionality: Optionality,
}

impl InlineSchema {
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        let all_of = if let Some(all_of) = object.all_of.clone() {
            AllOfItemShape::from_schema_cases(all_of).to_result()?
        } else {
            vec![]
        };
        let one_of = if let Some(one_of) = object.one_of.clone() {
            OneOfItemShape::from_schema_cases(one_of).to_result()?
        } else {
            vec![]
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
    pub fn pop_all_of_if_single_ref(&self) -> Result<Option<RefShape>> {
        let ref_shape = match self.all_of.as_slice() {
            [AllOfItemShape::Ref(object)] => object,
            _ => return Ok(None),
        };
        let mut ref_shape = ref_shape.clone();
        ref_shape.is_required = self.optionality.is_required;
        ref_shape.nullable = Some(self.optionality.is_nullable);

        Ok(Some(ref_shape.clone()))
    }
}
