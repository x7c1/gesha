mod to_struct;
use to_struct::to_struct;

mod type_factory;
use type_factory::TypeFactory;

use crate::conversions::{Result, ToRustType};
use crate::targets::rust_type::{Definition, ModuleName, Modules, NewTypeDef};
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
    use OpenApiDataType as ot;
    match object.data_type.as_ref() {
        Some(ot::Object) => to_struct(name, object),
        Some(ot::String | ot::Integer | ot::Number | ot::Boolean | ot::Array) => {
            to_newtype(name, object)
        }
        // define it as 'object' if 'type' is not specified.
        None => to_struct(name, object),
    }
}

fn to_newtype(name: SchemaFieldName, object: SchemaObject) -> Result<Definition> {
    let to_type = TypeFactory {
        format: object.format,
        items: object.items,
    };
    let openapi_type = object.data_type.unwrap_or_else(|| unimplemented!());
    let def = NewTypeDef {
        name: name.into(),
        data_type: to_type.apply(openapi_type)?,
    };
    Ok(def.into())
}
