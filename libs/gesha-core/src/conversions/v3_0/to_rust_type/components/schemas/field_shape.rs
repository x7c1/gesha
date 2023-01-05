use crate::conversions::v3_0::to_rust_type::components::schemas::TypeShape;
use openapi_types::v3_0::ComponentName;

#[derive(Clone, Debug)]
pub struct FieldShape {
    pub name: ComponentName,
    pub type_shape: TypeShape,
}
