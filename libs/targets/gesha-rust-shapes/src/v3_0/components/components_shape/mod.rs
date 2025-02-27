mod transform_core;
use transform_core::transform_core;

mod transform_request_bodies;
use transform_request_bodies::transform_request_bodies;

mod transform_schemas;
use transform_schemas::transform_schemas;

use crate::v3_0::components::core::CoreShape;
use crate::v3_0::components::request_bodies::RequestBodiesShape;
use crate::v3_0::components::schemas::{SchemasShape, TypeShape};
use gesha_core::conversions::{with_key, Output, Result};
use gesha_rust_types::ModDef;
use openapi_types::core::OutputOptionOps;

#[derive(Clone, Debug)]
pub struct ComponentsShape {
    pub schemas: SchemasShape,
    pub request_bodies: RequestBodiesShape,
    pub core: CoreShape,
}

impl ComponentsShape {
    pub fn into_mod_defs(self) -> Result<Output<Vec<ModDef>>> {
        let this = transform(self)?;
        let (request_bodies, errors_of_request_bodies) = this
            .request_bodies
            .define()
            .maybe()
            .bind_errors(with_key("request_bodies"))
            .into_tuple();

        let (schemas, errors_of_schemas) = this
            .schemas
            .define()
            .maybe()
            .bind_errors(with_key("schemas"))
            .into_tuple();

        let (core, errors_of_core) = this
            .core
            .define()
            .maybe()
            .bind_errors(with_key("core"))
            .into_tuple();

        let mod_defs = vec![request_bodies, schemas, core]
            .into_iter()
            .flatten()
            .collect();

        let output = Output::new(mod_defs, vec![])
            .append(errors_of_request_bodies)
            .append(errors_of_schemas)
            .append(errors_of_core);

        Ok(output)
    }

    pub fn any_type(&self, f: impl Fn(&TypeShape) -> bool) -> bool {
        self.schemas.any_type(&f)
    }
}

fn transform(shapes: ComponentsShape) -> Result<ComponentsShape> {
    let maybe = Output::optionize(transform_schemas)(Some(shapes))
        .bind_errors(with_key("schemas"))
        .to_result()?;

    let maybe = Output::optionize(transform_request_bodies)(maybe)
        .bind_errors(with_key("request_bodies"))
        .to_result()?;

    let shape = Output::optionize(transform_core)(maybe)
        .bind_errors(with_key("core"))
        .ok_or_errors()?;

    Ok(shape)
}
