mod components_shapes;
use components_shapes::ComponentsShapes;

mod from_request_bodies;
mod from_schemas;
mod post_processor;

use crate::conversions::{Result, ToRustType};
use crate::targets::rust_type::{DataType, Modules};
use openapi_types::v3_0::{ComponentsObject, Document};

impl ToRustType<Document> for Modules {
    fn apply(this: Document) -> Result<Self> {
        let module = this
            .components
            .map(ToRustType::apply)
            .unwrap_or_else(|| Ok(Modules::empty()))?;

        Ok(module)
    }
}

impl ToRustType<ComponentsObject> for Modules {
    fn apply(this: ComponentsObject) -> Result<Self> {
        let schemas = this
            .schemas
            .map(from_schemas::to_shapes)
            .unwrap_or_else(|| Ok(vec![]))?;

        let request_bodies = this
            .request_bodies
            .map(from_request_bodies::to_shapes)
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
