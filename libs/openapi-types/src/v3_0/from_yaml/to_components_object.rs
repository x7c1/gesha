use crate::core::OutputOptionOps;
use crate::v3_0::{ComponentsObject, RequestBodiesObject, SchemasObject};
use crate::yaml::{ToOpenApi, YamlMap};
use crate::{Output, Result, with_key};

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
