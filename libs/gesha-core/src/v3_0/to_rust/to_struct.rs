use crate::languages::rust::{Definition, StructField};
use openapi_types::v3_0::{SchemaCase, SchemaFieldName, SchemaObject, SchemaProperties};

pub(super) fn to_struct(name: SchemaFieldName, object: SchemaObject) -> crate::Result<Definition> {
    println!(".....{:#?}", object);

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
        SchemaCase::Schema(schema_object) => match schema_object.type_name {
            None => Err(crate::Error::FieldTypeMissing),
            Some(type_name) => Ok(StructField {
                name: field_name.into(),
                type_name,
            }),
        },
        SchemaCase::Reference(reference_object) => Err(crate::Error::todo(format!(
            "reference field not implemented: {:?}",
            reference_object
        ))),
    }
}
