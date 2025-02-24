use crate::v3_0::from_yaml::to_request_body_pair;
use crate::v3_0::from_yaml::to_schema_pair;
use crate::v3_0::{ComponentsObject, SchemasObject};
use crate::yaml::{collect, ToOpenApi, YamlMap};
use crate::{Error, Output, Result};

impl ToOpenApi for ComponentsObject {
    fn apply(mut map: YamlMap) -> Result<Output<Self>> {
        let (schemas, errors1): (Option<SchemasObject>, Vec<Error>) = map
            .remove_if_exists("schemas")?
            .map(collect(to_schema_pair))
            .map(|(x, y)| (Some(x), y))
            .unwrap_or((None, vec![]));

        let (request_bodies, errors2) = map
            .remove_if_exists("requestBodies")?
            .map(collect(to_request_body_pair))
            .map(|(x, y)| (Some(x), y))
            .unwrap_or((None, vec![]));

        let object = ComponentsObject {
            request_bodies,
            schemas,
        };
        Ok(Output::new(object, errors1).append(errors2))
    }
}
