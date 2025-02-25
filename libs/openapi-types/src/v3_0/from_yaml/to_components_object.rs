use crate::core::OutputOptionOps;
use crate::v3_0::from_yaml::to_request_body_pair;
use crate::v3_0::from_yaml::to_schema_pair;
use crate::v3_0::ComponentsObject;
use crate::yaml::{collect, ToOpenApi, YamlMap};
use crate::{with_key, Output, Result};

impl ToOpenApi for ComponentsObject {
    fn apply(mut map: YamlMap) -> Result<Output<Self>> {
        let (schemas, schemas_errors) = map
            .remove_if_exists("schemas")?
            .map(collect(|x| to_schema_pair(x).map(Output::no_error)))
            .maybe()
            .bind_errors(with_key("schemas"))
            .to_tuple();

        let (request_bodies, request_bodies_errors) = map
            .remove_if_exists("requestBodies")?
            .map(collect(|x| to_request_body_pair(x).map(Output::no_error)))
            .maybe()
            .bind_errors(with_key("requestBodies"))
            .to_tuple();

        let object = ComponentsObject {
            request_bodies,
            schemas,
        };
        let output = Output::new(object, schemas_errors).append(request_bodies_errors);
        Ok(output)
    }
}
