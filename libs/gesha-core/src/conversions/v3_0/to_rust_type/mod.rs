mod components;
use components::ComponentsShapes;

use crate::conversions::v3_0::to_rust_type::components::request_bodies::to_request_bodies_shape;
use crate::conversions::v3_0::to_rust_type::components::schemas::to_schemas_shape;
use crate::conversions::{Result, ToRustType};
use crate::targets::rust_type::{DataType, Modules};
use openapi_types::v3_0::ComponentsObject;

impl ToRustType<ComponentsObject> for Modules {
    fn apply(this: ComponentsObject) -> Result<Self> {
        let schemas = this
            .schemas
            .map(to_schemas_shape)
            .unwrap_or_else(|| Ok(vec![]))?;

        let request_bodies = this
            .request_bodies
            .map(to_request_bodies_shape)
            .unwrap_or_else(|| Ok(vec![]))?;

        let shapes = ComponentsShapes {
            schemas,
            request_bodies,
        };
        shapes.into_modules()
    }
}

pub fn contains_patch(x: &DataType) -> bool {
    match x {
        DataType::Bool => false,
        DataType::Int32 => false,
        DataType::Int64 => false,
        DataType::Float32 => false,
        DataType::Float64 => false,
        DataType::Option(x) => contains_patch(x),
        DataType::Patch(_) => true,
        DataType::String => false,
        DataType::Vec(x) => contains_patch(x),
        DataType::Custom(_) => false,
    }
}

pub fn is_patch(x: &DataType) -> bool {
    matches!(x, DataType::Patch(_))
}
