use crate::v3_0::from_yaml::to_request_body_pair;
use crate::v3_0::from_yaml::to_schema_pair;
use crate::v3_0::{ComponentsObject, SchemasObject};
use crate::yaml::{collect, ToOpenApi, YamlMap};
use crate::{Error, Result};

impl ToOpenApi for ComponentsObject {
    fn apply(mut map: YamlMap) -> Result<Self> {
        let (schemas, errors1): (Option<SchemasObject>, Vec<Error>) = map
            .remove_if_exists("schemas")?
            .map(collect(to_schema_pair))
            .map(|(x, y)| (Some(x), y))
            .unwrap_or((None, vec![]));

        // TODO: return error with self
        println!("detected errors1: {:#?}", errors1);

        let (request_bodies, errors2) = map
            .remove_if_exists("requestBodies")?
            .map(collect(to_request_body_pair))
            .map(|(x, y)| (Some(x), y))
            .unwrap_or((None, vec![]));

        // TODO: return error with self
        println!("detected errors2: {:#?}", errors2);

        Ok(ComponentsObject {
            request_bodies,
            schemas,
        })
    }
}
