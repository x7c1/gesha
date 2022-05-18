mod to_struct;
use to_struct::to_struct;

use crate::rust_types::{Definition, ModuleName, RustModules};
use indexmap::indexmap;
use openapi_types::v3_0::{
    ComponentsObject, SchemaCase, SchemaFieldName, SchemaObject, SchemasObject,
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
        SchemaCase::Schema(obj) => to_rust_type(field_name, obj),
        SchemaCase::Reference(_) => todo!(),
    }
}

fn to_rust_type(name: SchemaFieldName, object: SchemaObject) -> crate::Result<Definition> {
    match object.type_name.as_deref() {
        Some("object") => to_struct(name, object),
        Some("array") => to_vec(name, object),
        // TODO:
        _ => unimplemented!(),
    }
}

fn to_vec(name: SchemaFieldName, _object: SchemaObject) -> crate::Result<Definition> {
    Ok(Definition::VecDef {
        name: name.into(),
        // TODO:
        type_name: "todo".to_string(),
    })
}
