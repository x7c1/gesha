use crate::v3_0::{reify_entry, reify_value};
use crate::yaml_wrapper::{YamlArray, YamlMap};
use indexmap::IndexSet;
use openapi_types::v3_0::{
    ComponentsObject, ReferenceObject, RequiredSchemaFields, SchemaCase, SchemaFieldName,
    SchemaObject, SchemaProperties, SchemasObject,
};
use std::collections::HashMap;

pub fn to_components_object(mut map: YamlMap) -> crate::Result<ComponentsObject> {
    let schemas = map
        .remove_if_exists("schemas")?
        .map(to_schemas)
        .transpose()?;

    Ok(ComponentsObject { schemas })
}

fn to_schemas(map: YamlMap) -> crate::Result<SchemasObject> {
    let schema_map = map
        .into_iter()
        .map(reify_entry)
        .collect::<crate::Result<Vec<(String, YamlMap)>>>()?
        .into_iter()
        .map(to_schema_pair)
        .collect::<crate::Result<HashMap<SchemaFieldName, SchemaCase>>>()?;

    Ok(SchemasObject::new(schema_map))
}

fn to_schema_pair(kv: (String, YamlMap)) -> crate::Result<(SchemaFieldName, SchemaCase)> {
    let (name, map) = kv;
    Ok((SchemaFieldName::new(name), to_schema_case(map)?))
}

fn to_schema_case(mut map: YamlMap) -> crate::Result<SchemaCase> {
    let case = match map.remove_if_exists::<String>("$ref")? {
        Some(reference) => SchemaCase::Reference(ReferenceObject::new(reference)),
        None => SchemaCase::Schema(to_schema_object(map)?),
    };
    Ok(case)
}

fn to_schema_object(mut map: YamlMap) -> crate::Result<SchemaObject> {
    let properties = map
        .remove_if_exists("properties")?
        .map(to_properties)
        .transpose()?;

    let required = map
        .remove_if_exists::<YamlArray>("required")?
        .map(to_required)
        .transpose()?;

    Ok(SchemaObject {
        type_name: map.remove_if_exists::<String>("type")?,
        properties,
        required,
    })
}

fn to_properties(map: YamlMap) -> crate::Result<SchemaProperties> {
    map.into_iter()
        .map(reify_entry)
        .collect::<crate::Result<Vec<(String, YamlMap)>>>()?
        .into_iter()
        .map(to_schema_pair)
        .collect()
}

fn to_required(array: YamlArray) -> crate::Result<RequiredSchemaFields> {
    let fields = array
        .into_iter()
        .map(reify_value)
        .collect::<crate::Result<IndexSet<String>>>()?;

    Ok(RequiredSchemaFields::new(fields))
}
