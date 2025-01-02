use crate::v3_0::components::core::CoreShape;
use crate::v3_0::components::request_bodies::RequestBodiesShape;
use crate::v3_0::components::schemas::SchemasShape;
use crate::v3_0::components::ComponentsShape;
use crate::{Result, ToRustType};
use gesha_rust_types::SourceCode;
use openapi_types::v3_0::{ComponentsObject, Document};

impl ToRustType for Document {
    fn apply(self) -> Result<SourceCode> {
        let module = self
            .components
            .map(ToRustType::apply)
            .unwrap_or_else(|| Ok(SourceCode::empty()))?;

        Ok(module)
    }
}

impl ToRustType for ComponentsObject {
    fn apply(self) -> Result<SourceCode> {
        let shapes = ComponentsShape {
            schemas: SchemasShape::shape(self.schemas)?,
            request_bodies: RequestBodiesShape::shape(self.request_bodies)?,
            core: CoreShape::default(),
        };
        shapes.into_source_code()
    }
}
