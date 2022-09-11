use crate::conversions::v3_0::to_openapi::{to_request_body_pair, to_schema_pair};
use crate::conversions::{reify_entry, Result, ToOpenApi};
use crate::yaml::YamlMap;
use indexmap::IndexMap;
use openapi_types::v3_0::{
    ComponentName, ComponentsObject, RequestBodiesObject, RequestBodyCase, SchemasObject,
};

impl ToOpenApi for ComponentsObject {
    fn apply(mut map: YamlMap) -> Result<Self> {
        let schemas = map
            .remove_if_exists("schemas")?
            .map(ToOpenApi::apply)
            .transpose()?;

        let request_bodies = map
            .remove_if_exists("requestBodies")?
            .map(ToOpenApi::apply)
            .transpose()?;

        Ok(ComponentsObject {
            request_bodies,
            schemas,
        })
    }
}

impl ToOpenApi for RequestBodiesObject {
    fn apply(map: YamlMap) -> Result<Self> {
        let cases = map
            .into_iter()
            .map(reify_entry)
            .collect::<Result<Vec<(String, YamlMap)>>>()?
            .into_iter()
            .map(to_request_body_pair)
            .collect::<Result<IndexMap<ComponentName, RequestBodyCase>>>()?;

        Ok(RequestBodiesObject::new(cases))
    }
}

impl ToOpenApi for SchemasObject {
    fn apply(map: YamlMap) -> Result<Self> {
        map.into_iter()
            .map(reify_entry)
            .collect::<Result<Vec<(String, YamlMap)>>>()?
            .into_iter()
            .map(to_schema_pair)
            .collect()
    }
}
