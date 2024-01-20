use crate::v3_0::from_yaml::to_request_body_pair;
use crate::v3_0::from_yaml::to_schema_pair;
use crate::v3_0::ComponentsObject;
use crate::yaml::{collect, ToOpenApi, YamlMap};
use crate::Result;

impl ToOpenApi for ComponentsObject {
    fn apply(mut map: YamlMap) -> Result<Self> {
        let schemas = map
            .remove_if_exists("schemas")?
            .map(collect(to_schema_pair))
            .transpose()?;

        let request_bodies = map
            .remove_if_exists("requestBodies")?
            .map(collect(to_request_body_pair))
            .transpose()?;

        Ok(ComponentsObject {
            request_bodies,
            schemas,
        })
    }
}
