use crate::yaml_wrapper::YamlMap;
use openapi_types::v3_0::{
    ComponentsObject, ObjectTypeSchema, ReferenceObject, SchemaCase, SchemaFieldName, SchemaObject,
    SchemasObject,
};
use std::collections::HashMap;

pub fn to_components_object(map: YamlMap) -> crate::Result<ComponentsObject> {
    println!("to_components_object: {:#?}", map);

    // TODO:
    let sample_schema = SchemaCase::Schema(SchemaObject::Object(ObjectTypeSchema {
        properties: vec![SchemaCase::Reference(ReferenceObject {})],
    }));
    let sample_name = SchemaFieldName::new("Sample");

    let mut map = HashMap::default();
    map.insert(sample_name, sample_schema);

    Ok(ComponentsObject {
        // TODO:
        schemas: SchemasObject::new(map),
    })
}
