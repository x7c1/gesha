use crate::conversions::Error::UnknownDataType;
use crate::conversions::{reify_entry, reify_value, Result, ToOpenApi};
use crate::yaml::{YamlArray, YamlMap};
use indexmap::IndexSet;
use openapi_types::v3_0::{
    ArrayItems, ComponentsObject, FormatModifier, OpenApiDataType, ReferenceObject,
    RequiredSchemaFields, SchemaCase, SchemaFieldName, SchemaObject, SchemaProperties,
    SchemasObject,
};

impl ToOpenApi for ComponentsObject {
    fn apply(mut map: YamlMap) -> Result<Self> {
        let schemas = map
            .remove_if_exists("schemas")?
            .map(ToOpenApi::apply)
            .transpose()?;

        Ok(ComponentsObject { schemas })
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

fn to_schema_pair(kv: (String, YamlMap)) -> Result<(SchemaFieldName, SchemaCase)> {
    let (name, map) = kv;
    Ok((SchemaFieldName::new(name), to_schema_case(map)?))
}

fn to_schema_case(mut map: YamlMap) -> Result<SchemaCase> {
    let case = match map.remove_if_exists::<String>("$ref")? {
        Some(reference) => SchemaCase::Reference(ReferenceObject::new(reference)),
        None => SchemaCase::Schema(to_schema_object(map)?),
    };
    Ok(case)
}

fn to_schema_object(mut map: YamlMap) -> Result<SchemaObject> {
    let properties = map
        .remove_if_exists("properties")?
        .map(to_properties)
        .transpose()?;

    let required = map
        .remove_if_exists::<YamlArray>("required")?
        .map(to_required)
        .transpose()?;

    let data_type = map
        .remove_if_exists::<String>("type")?
        .map(to_data_type)
        .transpose()?;

    let format = map
        .remove_if_exists::<String>("format")?
        .map(to_format_modifier)
        .transpose()?;

    let items = map
        .remove_if_exists::<YamlMap>("items")?
        .map(to_array_items)
        .transpose()?;

    Ok(SchemaObject {
        data_type,
        format,
        properties,
        required,
        items,
    })
}

fn to_properties(map: YamlMap) -> Result<SchemaProperties> {
    map.into_iter()
        .map(reify_entry)
        .collect::<Result<Vec<(String, YamlMap)>>>()?
        .into_iter()
        .map(to_schema_pair)
        .collect()
}

fn to_required(array: YamlArray) -> Result<RequiredSchemaFields> {
    let fields = array
        .into_iter()
        .map(reify_value)
        .collect::<Result<IndexSet<String>>>()?;

    Ok(RequiredSchemaFields::new(fields))
}

fn to_data_type(x: String) -> Result<OpenApiDataType> {
    OpenApiDataType::find(&x).ok_or(UnknownDataType(x))
}

fn to_format_modifier(x: String) -> Result<FormatModifier> {
    Ok(FormatModifier::find(&x).unwrap_or(FormatModifier::Custom(x)))
}

fn to_array_items(map: YamlMap) -> Result<ArrayItems> {
    let case = to_schema_case(map)?;
    Ok(ArrayItems::new(case))
}
