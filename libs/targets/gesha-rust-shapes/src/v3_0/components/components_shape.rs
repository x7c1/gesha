use crate::v3_0::components::core::CoreShape;
use crate::v3_0::components::request_bodies::RequestBodiesShape;
use crate::v3_0::components::schemas::{SchemasShape, TypeShape};
use gesha_core::conversions::{Output, with_key};
use gesha_rust_types::ModDef;
use openapi_types::core::OutputOptionOps;

#[derive(Clone, Debug)]
pub struct ComponentsShape {
    pub schemas: SchemasShape,
    pub request_bodies: RequestBodiesShape,
    pub core: CoreShape,
}

impl ComponentsShape {
    pub fn define(self) -> Output<Vec<ModDef>> {
        let (request_bodies, errors_of_request_bodies) = self
            .request_bodies
            .define()
            .maybe()
            .bind_errors(with_key("request_bodies"))
            .into_tuple();

        let (schemas, errors_of_schemas) = self
            .schemas
            .define()
            .maybe()
            .bind_errors(with_key("schemas"))
            .into_tuple();

        let (core, errors_of_core) = self
            .core
            .define()
            .maybe()
            .bind_errors(with_key("core"))
            .into_tuple();

        let mod_defs = vec![request_bodies, schemas, core]
            .into_iter()
            .flatten()
            .collect();

        Output::ok(mod_defs)
            .append(errors_of_request_bodies)
            .append(errors_of_schemas)
            .append(errors_of_core)
    }

    pub fn any_type(&self, f: impl Fn(&TypeShape) -> bool) -> bool {
        self.schemas.any_type(&f)
    }
}
