mod components_shapes;
use components_shapes::ComponentsShapes;

mod object_to_field_shapes;
use object_to_field_shapes::object_to_field_shapes;

mod post_process;
use post_process::PostProcessor;

mod shape_schema_object_type;
use shape_schema_object_type::shape_schema_object_type;

mod shape_type;
use shape_type::shape_type;

mod to_struct;
use to_struct::to_struct;

use crate::conversions::v3_0::to_rust_type::DefinitionShape::Fixed;
use crate::conversions::{Result, ToRustType};
use crate::targets::rust_type::{
    DataType, Definition, EnumDef, EnumVariant, Modules, NewTypeDef, StructField, StructFieldName,
};
use openapi_types::v3_0::{
    AllOf, ComponentsObject, Document, EnumValues, OpenApiDataType, ReferenceObject, SchemaCase,
    SchemaFieldName, SchemaObject, SchemasObject,
};
use DefinitionShape::InProcess;

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
        let to_shapes = |object: SchemasObject| {
            object
                .into_iter()
                .map(from_schema_entry)
                .collect::<Result<Vec<DefinitionShape>>>()
        };
        let mut shapes = ComponentsShapes {
            schemas: this.schemas.map(to_shapes).unwrap_or_else(|| Ok(vec![]))?,
        };
        PostProcessor::run(&mut shapes)?;
        shapes.into_modules()
    }
}

fn from_schema_entry(kv: (SchemaFieldName, SchemaCase)) -> Result<DefinitionShape> {
    let (field_name, schema_case) = kv;
    match schema_case {
        SchemaCase::Schema(obj) => to_definition(field_name, *obj),
        SchemaCase::Reference(_) => todo!(),
    }
}

fn to_definition(name: SchemaFieldName, object: SchemaObject) -> Result<DefinitionShape> {
    if let Some(all_of) = object.all_of {
        return to_all_of(name, all_of);
    }

    use OpenApiDataType as ot;
    match object.data_type.as_ref() {
        Some(ot::Object) => to_struct(name, object),
        Some(ot::String) => match object.enum_values {
            Some(values) => to_enum(name, values),
            None => to_newtype(name, object),
        },
        Some(ot::Integer | ot::Number | ot::Boolean | ot::Array) => to_newtype(name, object),

        // define it as 'object' if 'type' is not specified.
        None => to_struct(name, object),
    }
}

fn to_newtype(name: SchemaFieldName, object: SchemaObject) -> Result<DefinitionShape> {
    match shape_schema_object_type(object)? {
        TypeShape::Fixed(data_type) => {
            let def = NewTypeDef {
                name: name.into(),
                data_type,
            };
            Ok(Fixed(def.into()))
        }
        TypeShape::Vec(_) => unimplemented!(),
    }
}

fn to_enum(name: SchemaFieldName, values: EnumValues) -> Result<DefinitionShape> {
    let def = EnumDef {
        name: name.into(),
        variants: values.into_iter().map(EnumVariant::new).collect(),
    };
    Ok(Fixed(def.into()))
}

fn to_all_of(name: SchemaFieldName, cases: AllOf) -> Result<DefinitionShape> {
    let shapes = cases
        .into_iter()
        .map(to_all_of_item_shape)
        .collect::<Result<Vec<AllOfItemShape>>>()?;

    let process = PostProcess::AllOf {
        name: name.into(),
        shapes,
    };
    Ok(process.into())
}

fn to_all_of_item_shape(case: SchemaCase) -> Result<AllOfItemShape> {
    let shape = match case {
        SchemaCase::Schema(object) => {
            let shapes = object_to_field_shapes(*object)?;
            AllOfItemShape::Object(shapes)
        }
        SchemaCase::Reference(x) => AllOfItemShape::Ref(x),
    };
    Ok(shape)
}

#[derive(Clone, Debug)]
enum DefinitionShape {
    Fixed(Definition),
    InProcess(PostProcess),
}

#[derive(Clone, Debug)]
enum PostProcess {
    AllOf {
        name: String,
        shapes: Vec<AllOfItemShape>,
    },
}

impl From<PostProcess> for DefinitionShape {
    fn from(this: PostProcess) -> Self {
        InProcess(this)
    }
}

#[derive(Clone, Debug)]
enum AllOfItemShape {
    Object(Vec<FieldShape>),
    Ref(ReferenceObject),
}

#[derive(Clone, Debug)]
pub enum TypeShape {
    Fixed(DataType),
    Vec(Box<TypeShape>),
}

#[derive(Clone, Debug)]
enum FieldShape {
    Fixed(StructField),
    InProcess {
        name: StructFieldName,
        type_shape: TypeShape,
    },
}
