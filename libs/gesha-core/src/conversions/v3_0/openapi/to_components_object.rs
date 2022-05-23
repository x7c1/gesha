use crate::conversions::{reify_entry, reify_value, ToOpenApi};
use crate::yaml_wrapper::{YamlArray, YamlMap};
use indexmap::IndexSet;
use openapi_types::v3_0::{
    ComponentsObject, OpenApiDataType, ReferenceObject, RequiredSchemaFields, SchemaCase,
    SchemaFieldName, SchemaObject, SchemaProperties, SchemasObject,
};

pub(super) fn to_components_object(mut map: YamlMap) -> crate::Result<ComponentsObject> {
    let schemas = map
        .remove_if_exists("schemas")?
        .map(ToOpenApi::apply)
        .transpose()?;

    Ok(ComponentsObject { schemas })
}

impl ToOpenApi for SchemasObject {
    fn apply(map: YamlMap) -> crate::Result<Self> {
        map.into_iter()
            .map(reify_entry)
            .collect::<crate::Result<Vec<(String, YamlMap)>>>()?
            .into_iter()
            .map(to_schema_pair)
            .collect()
    }
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

    let date_type = map
        .remove_if_exists::<String>("type")?
        .map(to_date_type)
        .transpose()?;

    Ok(SchemaObject {
        data_type: date_type,
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

fn to_date_type(x: String) -> crate::Result<OpenApiDataType> {
    match x.as_str() {
        "object" => Ok(OpenApiDataType::Object),
        "string" => Ok(OpenApiDataType::String),
        "integer" => Ok(OpenApiDataType::Integer),
        "array" => Ok(OpenApiDataType::Array),
        _ => Err(crate::Error::UnknownDataType(x)),
    }
}
