use crate::targets::rust_type::{Definition, FieldType, StructField};
use openapi_types::v3_0::{
    OpenApiDataType, SchemaCase, SchemaFieldName, SchemaObject, SchemaProperties,
};

pub(super) fn to_struct(name: SchemaFieldName, object: SchemaObject) -> crate::Result<Definition> {
    let fields = object.properties.map(to_fields).unwrap_or(Ok(vec![]))?;
    Ok(Definition::StructDef {
        name: name.into(),
        fields,
    })
}

fn to_fields(props: SchemaProperties) -> crate::Result<Vec<StructField>> {
    props.into_iter().map(to_field).collect()
}

fn to_field(entry: (SchemaFieldName, SchemaCase)) -> crate::Result<StructField> {
    let (field_name, schema_case) = entry;
    match schema_case {
        SchemaCase::Schema(schema_object) => match schema_object.data_type {
            Some(data_type) => Ok(StructField {
                name: field_name.into(),
                data_type: to_field_type(data_type)?,
            }),
            None => Err(crate::Error::FieldTypeMissing),
        },
        // TODO:
        SchemaCase::Reference(reference_object) => Err(crate::Error::todo(format!(
            "reference field not implemented: {:?}",
            reference_object
        ))),
    }
}

fn to_field_type(data_type: OpenApiDataType) -> crate::Result<FieldType> {
    match data_type {
        OpenApiDataType::String => Ok(FieldType::String),
        // TODO: receive "format"
        OpenApiDataType::Integer => Ok(FieldType::Int64),
        // TODO: receive "items"
        OpenApiDataType::Array => Ok(FieldType::Vec),
        OpenApiDataType::Object => Err(crate::Error::todo(format!(
            "inline object definition not implemented: {:?}",
            data_type
        ))),
    }
}
