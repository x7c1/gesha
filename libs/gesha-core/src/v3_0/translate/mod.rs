use crate::rust_types::{ComponentsType, SchemaType};
use openapi_types::v3_0::{ComponentsObject, SchemaCase, SchemaFieldName, SchemasObject};

pub fn from_components(components: ComponentsObject) -> crate::Result<ComponentsType> {
    let schemas = components
        .schemas
        .map(from_schemas)
        .unwrap_or_else(|| Ok(vec![]))?;

    Ok(ComponentsType { schemas })
}

fn from_schemas(schemas: SchemasObject) -> crate::Result<Vec<SchemaType>> {
    schemas.into_iter().map(from_schema_entry).collect()
}

fn from_schema_entry(kv: (SchemaFieldName, SchemaCase)) -> crate::Result<SchemaType> {
    println!(">{:#?}", kv);
    let (field_name, _schema_case) = kv;

    Ok(SchemaType {
        name: field_name.into(),
    })
}
