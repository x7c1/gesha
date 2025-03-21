use crate::core::{OutputMergeOps, OutputOptionOps};
use crate::v3_0::{
    ArrayItems, ComponentName, EnumValue, FormatModifier, OpenApiDataType, ReferenceObject,
    RequiredSchemaFields, SchemaCase, SchemaObject, SchemaProperties,
};
use crate::yaml::{YamlArray, YamlMap, collect, reify_value};
use crate::{Error, Output, Result, by_key, with_key};

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
        .remove_if_exists::<YamlMap>("properties")
        .maybe()
        .flat_map_if_some(to_properties)
        .bind_errors(with_key("properties"))
        .into_tuple();

    let (required, errors_of_required) = map
        .remove_if_exists::<YamlArray>("required")
        .maybe()
        .try_map_if_some(RequiredSchemaFields::from_yaml_array)
        .bind_errors(with_key("required"))
        .into_tuple();

    let (data_type, errors_of_data_type) = map
        .remove_if_exists::<String>("type")
        .maybe()
        .try_map_if_some(OpenApiDataType::new)
        .bind_errors(with_key("type"))
        .into_tuple();

    let (format, errors_of_format) = map
        .remove_if_exists::<String>("format")
        .maybe()
        .try_map_if_some(to_format_modifier)
        .bind_errors(with_key("format"))
        .into_tuple();

    let (nullable, errors_of_nullable) = map
        .remove_if_exists::<bool>("nullable")
        .maybe()
        .bind_errors(with_key("nullable"))
        .into_tuple();

    let (items, errors_of_items) = map
        .remove_if_exists::<YamlMap>("items")
        .maybe()
        .try_map_if_some(to_array_items)
        .bind_errors(with_key("items"))
        .into_tuple();

    let (enum_values, errors_of_enum) = map
        .remove_if_exists::<YamlArray>("enum")
        .maybe()
        .try_map_if_some(EnumValue::from_yaml_array)
        .bind_errors(with_key("enum"))
        .into_tuple();

    let (all_of, errors_all_of) = map
        .remove_if_exists::<YamlArray>("allOf")
        .maybe()
        .flat_map_if_some(to_schema_cases)
        .bind_errors(with_key("allOf"))
        .into_tuple();

    let (one_of, errors_one_of) = map
        .remove_if_exists::<YamlArray>("oneOf")
        .maybe()
        .flat_map_if_some(to_schema_cases)
        .bind_errors(with_key("oneOf"))
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
        all_of,
        one_of,
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

fn to_properties(map: YamlMap) -> Output<SchemaProperties> {
    collect(Output::by(to_schema_pair))(map)
}

fn to_format_modifier(x: String) -> Result<FormatModifier> {
    Ok(FormatModifier::find(&x).unwrap_or(FormatModifier::Custom(x)))
}

fn to_array_items(map: YamlMap) -> Result<ArrayItems> {
    let case = to_schema_case(map)?;
    let items = ArrayItems::new(case);
    Ok(items)
}

fn to_schema_cases(array: YamlArray) -> Output<Vec<SchemaCase>> {
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
