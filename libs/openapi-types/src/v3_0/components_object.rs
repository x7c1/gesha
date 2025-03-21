use crate::core::OutputOptionOps;
use crate::v3_0::{RequestBodiesObject, SchemasObject};
use crate::yaml::{ToOpenApi, YamlMap};
use crate::{Output, Result, with_key};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsObject
#[derive(Debug)]
pub struct ComponentsObject {
    pub request_bodies: Option<RequestBodiesObject>,
    pub schemas: Option<SchemasObject>,
}

impl ToOpenApi for ComponentsObject {
    fn apply(mut map: YamlMap) -> Result<Output<Self>> {
        let (schemas, schemas_errors) = map
            .remove_if_exists("schemas")?
            .map(SchemasObject::from_yaml_map)
            .maybe()
            .bind_errors(with_key("schemas"))
            .into_tuple();

        let (request_bodies, request_bodies_errors) = map
            .remove_if_exists("requestBodies")?
            .map(RequestBodiesObject::from_yaml_map)
            .maybe()
            .bind_errors(with_key("requestBodies"))
            .into_tuple();

        let object = ComponentsObject {
            request_bodies,
            schemas,
        };
        let output = Output::new(object, schemas_errors).append(request_bodies_errors);
        Ok(output)
    }
}
