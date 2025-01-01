mod transform_core;
use transform_core::transform_core;

mod transform_request_bodies;
use transform_request_bodies::transform_request_bodies;

mod transform_schemas;
use transform_schemas::transform_schemas;

use crate::v3_0::components::core::CoreShape;
use crate::v3_0::components::request_bodies::RequestBodiesShape;
use crate::v3_0::components::schemas::{SchemasShape, TypeShape};
use crate::Result;
use gesha_rust_types::SourceCode;

#[derive(Clone, Debug)]
pub struct ComponentsShape {
    pub schemas: SchemasShape,
    pub request_bodies: RequestBodiesShape,
    pub core: CoreShape,
}

impl ComponentsShape {
    pub fn into_source_code(self) -> Result<SourceCode> {
        let this = transform(self)?;
        let mod_defs = vec![
            this.request_bodies.define()?,
            this.schemas.define()?,
            this.core.define()?,
        ]
        .into_iter()
        .flatten()
        .collect();

        // TODO: add preamble
        let code = SourceCode::new(mod_defs);
        Ok(code)
    }

    pub fn any_type(&self, f: impl Fn(&TypeShape) -> bool) -> bool {
        self.schemas.any_type(&f)
    }
}

fn transform(shapes: ComponentsShape) -> Result<ComponentsShape> {
    let shapes = transform_schemas(shapes)?;
    let shapes = transform_request_bodies(shapes)?;
    let shapes = transform_core(shapes)?;
    Ok(shapes)
}
