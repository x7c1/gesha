use crate::v3_0::{RequestBodiesObject, SchemasObject};
use crate::yaml::{ToOpenApi, YamlMap};
use crate::{Output, Result};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsObject
#[derive(Debug)]
pub struct ComponentsObject {
    pub request_bodies: Option<RequestBodiesObject>,
    pub schemas: Option<SchemasObject>,
}

impl ToOpenApi for ComponentsObject {
    fn apply(mut map: YamlMap) -> Result<Output<Self>> {
        let (schemas, schemas_errors) = map
            .flat_extract_if_exists("schemas", SchemasObject::from_yaml_map)
            .into_tuple();

        let (request_bodies, request_bodies_errors) = map
            .flat_extract_if_exists("requestBodies", RequestBodiesObject::from_yaml_map)
            .into_tuple();

        let object = ComponentsObject {
            request_bodies,
            schemas,
        };

        let output = Output::ok(object)
            .append(schemas_errors)
            .append(request_bodies_errors);

        Ok(output)
    }
}
