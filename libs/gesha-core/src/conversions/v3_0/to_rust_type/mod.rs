mod components;

use crate::conversions::v3_0::to_rust_type::components::core::CoreShape;
use crate::conversions::v3_0::to_rust_type::components::request_bodies::RequestBodiesShape;
use crate::conversions::v3_0::to_rust_type::components::schemas::SchemasShape;
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::{Result, ToRustType};
use gesha_rust_types::Modules;
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
        let shapes = ComponentsShape {
            schemas: SchemasShape::shape(this.schemas)?,
            request_bodies: RequestBodiesShape::shape(this.request_bodies)?,
            core: CoreShape::default(),
        };
        shapes.into_modules()
    }
}
