use crate::conversions::v3_0::to_rust_type::from_schemas::DefinitionShape;
use openapi_types::v3_0::ComponentName;

#[derive(Clone, Debug)]
pub struct ModShape {
    pub name: ComponentName,
    pub defs: Vec<DefinitionShape>,
}
