use crate::conversions::reify::collect;
use crate::conversions::v3_0::to_openapi::{to_request_body_pair, to_schema_pair};
use crate::conversions::{Result, ToOpenApi};
use crate::yaml::YamlMap;
use openapi_types::v3_0::ComponentsObject;

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
