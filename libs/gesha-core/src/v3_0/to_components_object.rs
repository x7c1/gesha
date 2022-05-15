use crate::yaml_wrapper::YamlMap;
use openapi_types::v3_0::{ComponentsObject, SchemasObject};
use std::collections::HashMap;

pub fn to_components_object(map: YamlMap) -> crate::Result<ComponentsObject> {
    println!("to_components_object: {:#?}", map);
    Ok(ComponentsObject {
        // TODO:
        schemas: SchemasObject::new(HashMap::default()),
    })
}
