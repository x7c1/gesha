mod components;

use crate::conversions::v3_0::to_rust_type::components::request_bodies::RequestBodiesShape;
use crate::conversions::v3_0::to_rust_type::components::schemas::SchemasShape;
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::{Result, ToRustType};
use crate::targets::rust_type::{DataType, Modules};
use openapi_types::v3_0::ComponentsObject;

impl ToRustType<ComponentsObject> for Modules {
    fn apply(this: ComponentsObject) -> Result<Self> {
        let schemas = this
            .schemas
            .map(SchemasShape::from)
            .unwrap_or_else(|| Ok(SchemasShape::empty()))?;

        let request_bodies = this
            .request_bodies
            .map(RequestBodiesShape::from)
            .unwrap_or_else(|| Ok(RequestBodiesShape::empty()))?;

        let shapes = ComponentsShape {
            schemas,
            request_bodies,
            ..Default::default()
        };
        shapes.into_modules()
    }
}

fn contains_patch(x: &DataType) -> bool {
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

fn is_patch(x: &DataType) -> bool {
    matches!(x, DataType::Patch(_))
}
