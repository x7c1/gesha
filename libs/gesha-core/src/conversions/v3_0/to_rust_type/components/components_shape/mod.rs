mod transform_core;
use transform_core::transform_core;

mod transform_request_bodies;
use transform_request_bodies::transform_request_bodies;

mod transform_schemas;
use transform_schemas::transform_schemas;

use crate::conversions::v3_0::to_rust_type::components::core::CoreShape;
use crate::conversions::v3_0::to_rust_type::components::request_bodies::RequestBodiesShape;
use crate::conversions::v3_0::to_rust_type::components::schemas::{SchemasShape, TypeShape};
use crate::conversions::Result;
use crate::targets::rust_type::Modules;

#[derive(Clone, Debug)]
pub struct ComponentsShape {
    pub schemas: SchemasShape,
    pub request_bodies: RequestBodiesShape,
    pub core: CoreShape,
}

impl ComponentsShape {
    pub fn into_modules(self) -> Result<Modules> {
        let this = transform(self)?;
        let modules = vec![
            this.request_bodies.define()?,
            this.schemas.define()?,
            this.core.define()?,
        ]
        .into_iter()
        .flatten()
        .collect();

        Ok(modules)
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
