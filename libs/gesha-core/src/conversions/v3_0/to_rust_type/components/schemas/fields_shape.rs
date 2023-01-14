use crate::conversions::v3_0::to_rust_type::components::schemas::FieldShape;
use openapi_types::v3_0::RequiredSchemaFields;

#[derive(Debug, Clone)]
pub struct FieldsShape {
    pub required: Option<RequiredSchemaFields>,
    pub items: Vec<FieldShape>,
}
