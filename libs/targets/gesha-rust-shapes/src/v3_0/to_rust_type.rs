use crate::v3_0::components::core::CoreShape;
use crate::v3_0::components::request_bodies::RequestBodiesShape;
use crate::v3_0::components::schemas::SchemasShape;
use crate::v3_0::components::ComponentsShape;
use crate::{Result, ToRustType};
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
