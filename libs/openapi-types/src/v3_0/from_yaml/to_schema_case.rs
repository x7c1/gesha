use crate::core::OutputMergeOps;
use crate::v3_0::{
    AllOf, ArrayItems, ComponentName, EnumValues, FormatModifier, OneOf, OpenApiDataType,
    ReferenceObject, RequiredSchemaFields, SchemaCase, SchemaObject, SchemaProperties,
};
use crate::yaml::{YamlArray, YamlMap, reify_value};
use crate::{Error, Output, Result, by_key};

pub fn to_schema_pair(kv: (String, YamlMap)) -> Result<(ComponentName, SchemaCase)> {
    let (name, map) = kv;
    let pair = (
        ComponentName::new(&name),
        to_schema_case(map).map_err(by_key(name))?,
    );
    Ok(pair)
}

pub fn to_schema_case(mut map: YamlMap) -> Result<SchemaCase> {
    let case = match map.remove_if_exists::<String>("$ref")? {
        Some(reference) => SchemaCase::Reference(ReferenceObject::new(reference)),
        None => {
            let object = to_schema_object(map)?;
            SchemaCase::Schema(Box::new(object))
        }
    };
    Ok(case)
}

fn to_schema_object(mut map: YamlMap) -> Result<SchemaObject> {
    let (properties, errors_of_properties) = map
        .flat_extract_if_exists("properties", SchemaProperties::from_yaml_map)
        .into_tuple();

    let (required, errors_of_required) = map
        .try_extract_if_exists("required", RequiredSchemaFields::from_yaml_array)
        .into_tuple();

    let (data_type, errors_of_data_type) = map
        .try_extract_if_exists("type", OpenApiDataType::new)
        .into_tuple();

    let (format, errors_of_format) = map
        .try_extract_if_exists("format", FormatModifier::from_string)
        .into_tuple();

    let (nullable, errors_of_nullable) = map.extract_if_exists("nullable").into_tuple();

    let (items, errors_of_items) = map
        .try_extract_if_exists("items", ArrayItems::from_yaml_map)
        .into_tuple();

    let (enum_values, errors_of_enum) = map
        .try_extract_if_exists("enum", EnumValues::from_yaml_array)
        .into_tuple();

    let (all_of, errors_all_of) = map
        .flat_extract_if_exists("allOf", AllOf::from_yaml_array)
        .into_tuple();

    let (one_of, errors_one_of) = map
        .flat_extract_if_exists("oneOf", OneOf::from_yaml_array)
        .into_tuple();

    let object = SchemaObject {
        title: map.remove_if_exists::<String>("title")?,
        description: map.remove_if_exists::<String>("description")?,
        data_type,
        format,
        nullable,
        properties,
        required,
        items,
        enum_values,
        all_of: all_of.flatten(),
        one_of: one_of.flatten(),
    };
    let output = Output::ok(object)
        .append(errors_of_properties)
        .append(errors_of_required)
        .append(errors_of_data_type)
        .append(errors_of_format)
        .append(errors_of_nullable)
        .append(errors_of_items)
        .append(errors_of_enum)
        .append(errors_all_of)
        .append(errors_one_of);

    output.to_result().map_err(Error::multiple)
}

pub fn to_schema_cases(array: YamlArray) -> Output<Vec<SchemaCase>> {
    array
        .into_iter()
        .map(reify_value)
        .collect::<Vec<Result<YamlMap>>>()
        .merge()
        .map(|xs| {
            xs.into_iter()
                .map(to_schema_case)
                .collect::<Result<Vec<SchemaCase>>>()
                .merge()
        })
        .merge()
}
