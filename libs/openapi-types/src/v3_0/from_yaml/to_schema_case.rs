use crate::v3_0::{
    ArrayItems, ComponentName, EnumValues, FormatModifier, OpenApiDataType, ReferenceObject,
    RequiredSchemaFields, SchemaCase, SchemaObject, SchemaProperties,
};
use crate::yaml::{reify_entry, reify_value, YamlArray, YamlMap};
use crate::Error::UnknownDataType;
use crate::{Error, Result};
use indexmap::IndexSet;

pub fn to_schema_pair(kv: (String, YamlMap)) -> Result<(ComponentName, SchemaCase)> {
    let (name, map) = kv;
    Ok((
        ComponentName::new(&name),
        to_schema_case(map).map_err(Error::with_key(name))?,
    ))
}

pub fn to_schema_case(mut map: YamlMap) -> Result<SchemaCase> {
    let case = match map.remove_if_exists::<String>("$ref")? {
        Some(reference) => SchemaCase::Reference(ReferenceObject::new(reference)),
        None => SchemaCase::Schema(Box::new(to_schema_object(map)?)),
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

    let nullable = map.remove_if_exists::<bool>("nullable")?;

    let items = map
        .remove_if_exists::<YamlMap>("items")?
        .map(to_array_items)
        .transpose()?;

    let enum_values = map
        .remove_if_exists::<YamlArray>("enum")?
        .map(to_enum_values)
        .transpose()?;

    let all_of = map
        .remove_if_exists::<YamlArray>("allOf")?
        .map(to_schema_cases)
        .transpose()?;

    let one_of = map
        .remove_if_exists::<YamlArray>("oneOf")?
        .map(to_schema_cases)
        .transpose()?;

    Ok(SchemaObject {
        title: map.remove_if_exists::<String>("title")?,
        description: map.remove_if_exists::<String>("description")?,
        data_type,
        format,
        nullable,
        properties,
        required,
        items,
        enum_values,
        all_of,
        one_of,
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
    OpenApiDataType::find(&x).ok_or(UnknownDataType { found: x })
}

fn to_format_modifier(x: String) -> Result<FormatModifier> {
    Ok(FormatModifier::find(&x).unwrap_or(FormatModifier::Custom(x)))
}

fn to_array_items(map: YamlMap) -> Result<ArrayItems> {
    let case = to_schema_case(map)?;
    Ok(ArrayItems::new(case))
}

fn to_enum_values(array: YamlArray) -> Result<EnumValues> {
    array.into_iter().map(reify_value).collect()
}

fn to_schema_cases(array: YamlArray) -> Result<Vec<SchemaCase>> {
    array
        .into_iter()
        .map(reify_value)
        .collect::<Result<Vec<YamlMap>>>()?
        .into_iter()
        .map(to_schema_case)
        .collect()
}
