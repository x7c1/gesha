mod to_struct;
use to_struct::to_struct;

use crate::targets::rust::{Definition, ModuleName, RustModules};
use indexmap::indexmap;
use openapi_types::v3_0::{
    ComponentsObject, OpenApiDataType, SchemaCase, SchemaFieldName, SchemaObject, SchemasObject,
};

pub fn from_components(components: ComponentsObject) -> crate::Result<RustModules> {
    let schemas = components
        .schemas
        .map(from_schemas)
        .unwrap_or_else(|| Ok(vec![]))?;

    Ok(indexmap! {
         ModuleName::new("schemas") => schemas,
    })
}

fn from_schemas(schemas: SchemasObject) -> crate::Result<Vec<Definition>> {
    schemas.into_iter().map(from_schema_entry).collect()
}

fn from_schema_entry(kv: (SchemaFieldName, SchemaCase)) -> crate::Result<Definition> {
    let (field_name, schema_case) = kv;
    match schema_case {
        SchemaCase::Schema(obj) => to_definition(field_name, obj),
        SchemaCase::Reference(_) => todo!(),
    }
}

fn to_definition(name: SchemaFieldName, object: SchemaObject) -> crate::Result<Definition> {
    match object.data_type.as_ref() {
        Some(OpenApiDataType::Object) => to_struct(name, object),
        Some(OpenApiDataType::Array) => to_vec(name, object),
        _ => todo!(),
    }
}

fn to_vec(name: SchemaFieldName, object: SchemaObject) -> crate::Result<Definition> {
    println!("object.data_type: {:?}", object.data_type);
    Ok(Definition::VecDef {
        name: name.into(),
        // TODO: parse "items" field
        type_name: "todo".to_string(),
    })
}
