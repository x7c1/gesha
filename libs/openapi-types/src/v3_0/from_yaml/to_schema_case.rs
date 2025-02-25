use crate::core::{OutputMergeOps, OutputOptionOps};
use crate::v3_0::{
    ArrayItems, ComponentName, EnumValues, FormatModifier, OpenApiDataType, ReferenceObject,
    RequiredSchemaFields, SchemaCase, SchemaObject, SchemaProperties,
};
use crate::yaml::{collect, reify_value, YamlArray, YamlMap};
use crate::Error::UnknownDataType;
use crate::{by_key, with_key, Error, Output, Result};
use indexmap::IndexSet;

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
        .remove_if_exists("properties")?
        .map(to_properties)
        .maybe()
        .bind_errors(with_key("properties"))
        .to_tuple();

    // TODO: use Output
    let required = map
        .remove_if_exists::<YamlArray>("required")?
        .map(to_required)
        .transpose()?;

    let (data_type, errors_of_data_type) = map
        .remove_if_exists::<String>("type")?
        .map(to_data_type)
        .maybe()
        .to_tuple();

    // TODO: use Output
    let format = map
        .remove_if_exists::<String>("format")?
        .map(to_format_modifier)
        .transpose()?;

    let nullable = map.remove_if_exists::<bool>("nullable")?;

    let (items, errors_of_items) = map
        .remove_if_exists::<YamlMap>("items")?
        .map(to_array_items)
        .maybe()
        .bind_errors(with_key("items"))
        .to_tuple();

    // TODO: use Output
    let enum_values = map
        .remove_if_exists::<YamlArray>("enum")?
        .map(to_enum_values)
        .transpose()?;

    let (all_of, errors_all_of) = map
        .remove_if_exists::<YamlArray>("allOf")?
        .map(to_schema_cases)
        .maybe()
        .bind_errors(with_key("allOf"))
        .to_tuple();

    let (one_of, errors_one_of) = map
        .remove_if_exists::<YamlArray>("oneOf")?
        .map(to_schema_cases)
        .maybe()
        .bind_errors(with_key("oneOf"))
        .to_tuple();

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
    let output = Output::new(object, errors_of_properties)
        .append(errors_of_data_type)
        .append(errors_of_items)
        .append(errors_all_of)
        .append(errors_one_of);

    output.to_result().map_err(Error::multiple)
}

fn to_properties(map: YamlMap) -> Output<SchemaProperties> {
    collect(|x| to_schema_pair(x).map(Output::no_error))(map)
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
    let items = ArrayItems::new(case);
    Ok(items)
}

fn to_enum_values(array: YamlArray) -> Result<EnumValues> {
    array.into_iter().map(reify_value).collect()
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
