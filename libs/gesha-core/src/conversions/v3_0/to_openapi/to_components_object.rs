use crate::conversions::v3_0::to_openapi::{to_request_body_pair, to_schema_pair};
use crate::conversions::{reify_entry, Result, ToOpenApi};
use crate::yaml::YamlMap;
use openapi_types::v3_0::ComponentsObject;

impl ToOpenApi for ComponentsObject {
    fn apply(mut map: YamlMap) -> Result<Self> {
        let schemas = map
            .remove_if_exists("schemas")?
            .map(traverse(to_schema_pair))
            .transpose()?;

        let request_bodies = map
            .remove_if_exists("requestBodies")?
            .map(traverse(to_request_body_pair))
            .transpose()?;

        Ok(ComponentsObject {
            request_bodies,
            schemas,
        })
    }
}

fn traverse<X, Y, F>(f: F) -> impl FnOnce(YamlMap) -> Result<Y>
where
    F: Fn((String, YamlMap)) -> Result<X>,
    Y: FromIterator<X>,
{
    |map| {
        map.into_iter()
            .map(reify_entry)
            .collect::<Result<Vec<(String, YamlMap)>>>()?
            .into_iter()
            .map(f)
            .collect()
    }
}
