mod to_struct;
use to_struct::to_struct;

mod type_factory;

use crate::conversions::{Result, ToRustType};
use crate::targets::rust_type::{DataType, Definition, ModuleName, Modules, NewTypeDef, VecDef};
use indexmap::indexmap;
use openapi_types::v3_0::{
    ComponentsObject, Document, OpenApiDataType, SchemaCase, SchemaFieldName, SchemaObject,
    SchemasObject,
};

impl ToRustType<Document> for Modules {
    fn apply(this: Document) -> Result<Self> {
        this.components
            .map(ToRustType::apply)
            .unwrap_or_else(|| Ok(Modules::new()))
    }
}

impl ToRustType<ComponentsObject> for Modules {
    fn apply(this: ComponentsObject) -> Result<Self> {
        let schemas = this
            .schemas
            .map(ToRustType::apply)
            .unwrap_or_else(|| Ok(vec![]))?;

        Ok(indexmap! {
             ModuleName::new("schemas") => schemas,
        })
    }
}

impl ToRustType<SchemasObject> for Vec<Definition> {
    fn apply(this: SchemasObject) -> Result<Self> {
        this.into_iter().map(from_schema_entry).collect()
    }
}

fn from_schema_entry(kv: (SchemaFieldName, SchemaCase)) -> Result<Definition> {
    let (field_name, schema_case) = kv;
    match schema_case {
        SchemaCase::Schema(obj) => to_definition(field_name, obj),
        SchemaCase::Reference(_) => todo!(),
    }
}

fn to_definition(name: SchemaFieldName, object: SchemaObject) -> Result<Definition> {
    match object.data_type.as_ref() {
        Some(OpenApiDataType::Object) => to_struct(name, object),
        Some(OpenApiDataType::Array) => to_vec(name, object),
        Some(OpenApiDataType::String | OpenApiDataType::Integer) => to_newtype(name, object),
        _ => todo!("object.type: {:?}", object.data_type),
    }
}

fn to_newtype(name: SchemaFieldName, _object: SchemaObject) -> Result<Definition> {
    // _object.
    let def = NewTypeDef {
        name: name.into(),
        data_type: DataType::String,
    };
    Ok(def.into())
}

fn to_vec(name: SchemaFieldName, object: SchemaObject) -> Result<Definition> {
    println!("object.data_type: {:?}", object.data_type);
    let def = VecDef {
        name: name.into(),
        // TODO: parse "items" field
        type_name: "todo".to_string(),
    };
    Ok(def.into())
}
