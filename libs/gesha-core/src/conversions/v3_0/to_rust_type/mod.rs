mod to_struct;
use to_struct::{schema_object_to_data_type, to_struct};

mod post_process;
use post_process::post_process;

mod type_factory;

use crate::conversions::v3_0::to_rust_type::Fragment::Fixed;
use crate::conversions::Error::RequirePostProcess;
use crate::conversions::{Result, ToRustType};
use crate::targets::rust_type::{
    DataType, Definition, EnumDef, EnumVariant, ModuleName, Modules, NewTypeDef, StructField,
    StructFieldName,
};
use indexmap::indexmap;
use openapi_types::v3_0::{
    AllOf, ComponentsObject, Document, OpenApiDataType, SchemaCase, SchemaFieldName, SchemaObject,
    SchemasObject,
};
use Fragment::InProcess;

impl ToRustType<Document> for Modules {
    fn apply(this: Document) -> Result<Self> {
        let module = this
            .components
            .map(ToRustType::apply)
            .unwrap_or_else(|| Ok(Modules::new()))?;

        Ok(module)
    }
}

impl ToRustType<ComponentsObject> for Modules {
    fn apply(this: ComponentsObject) -> Result<Self> {
        let schemas = this
            .schemas
            .map(from_schemas_object)
            .unwrap_or_else(|| Ok(vec![]))?;

        let mut fragments = ComponentFragments { schemas };
        post_process(&mut fragments)?;
        fragments.into_modules()
    }
}

fn from_schemas_object(this: SchemasObject) -> Result<Vec<Fragment>> {
    this.into_iter().map(from_schema_entry).collect()
}

fn from_schema_entry(kv: (SchemaFieldName, SchemaCase)) -> Result<Fragment> {
    let (field_name, schema_case) = kv;
    match schema_case {
        SchemaCase::Schema(obj) => to_definition(field_name, *obj),
        SchemaCase::Reference(_) => todo!(),
    }
}

fn to_definition(name: SchemaFieldName, object: SchemaObject) -> Result<Fragment> {
    use OpenApiDataType as ot;
    match object.data_type.as_ref() {
        Some(ot::Object) => to_struct(name, object),
        Some(ot::String) if object.enum_values.is_some() => to_enum(name, object),
        Some(ot::String | ot::Integer | ot::Number | ot::Boolean | ot::Array) => {
            to_newtype(name, object)
        }
        _ if object.all_of.is_some() => reserve_all_of(name, object.all_of.unwrap()),

        // define it as 'object' if 'type' is not specified.
        None => to_struct(name, object),
    }
}

fn to_newtype(name: SchemaFieldName, object: SchemaObject) -> Result<Fragment> {
    match schema_object_to_data_type(object)? {
        FragmentType::Fixed(data_type) => {
            let def = NewTypeDef {
                name: name.into(),
                data_type,
            };
            Ok(Fixed(def.into()))
        }
        FragmentType::Vec(_) => unimplemented!(),
    }
}

fn to_enum(name: SchemaFieldName, object: SchemaObject) -> Result<Fragment> {
    let values = object.enum_values.expect("enum must be some");
    let variants = values.into_iter().map(EnumVariant::new).collect();
    let def = EnumDef {
        name: name.into(),
        variants,
    };
    Ok(Fixed(def.into()))
}

fn reserve_all_of(name: SchemaFieldName, cases: AllOf) -> Result<Fragment> {
    let process = PostProcess::AllOf {
        name: name.into(),
        cases,
    };
    Ok(process.into())
}

#[derive(Clone, Debug)]
struct ComponentFragments {
    schemas: Vec<Fragment>,
}

impl ComponentFragments {
    fn into_modules(self) -> Result<Modules> {
        let schemas = self
            .schemas
            .into_iter()
            .map(|x| match x {
                Fixed(def) => Ok(def),
                InProcess(process) => Err(RequirePostProcess(process)),
            })
            .collect::<Result<Vec<Definition>>>()?;

        Ok(indexmap! {
             ModuleName::new("schemas") => schemas,
        })
    }
}

#[derive(Clone, Debug)]
enum Fragment {
    Fixed(Definition),
    InProcess(PostProcess),
}

#[derive(Clone, Debug)]
pub enum PostProcess {
    AllOf { name: String, cases: AllOf },
}

impl From<PostProcess> for Fragment {
    fn from(this: PostProcess) -> Self {
        InProcess(this)
    }
}

#[derive(Clone, Debug)]
pub enum FragmentType {
    Fixed(DataType),
    Vec(Box<FragmentType>),
}

#[derive(Clone, Debug)]
pub enum FragmentStructField {
    Fixed(StructField),
    InProcess {
        name: StructFieldName,
        data_type: FragmentType,
    },
}
