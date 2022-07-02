use crate::conversions::v3_0::to_rust_type::definition_shape::DefinitionShape;
use crate::conversions::v3_0::to_rust_type::definition_shape::DefinitionShape::{Fixed, InProcess};
use crate::conversions::v3_0::to_rust_type::{AllOfItemShape, PostProcess, TypeShape};
use crate::conversions::Result;
use crate::targets::rust_type::{EnumDef, EnumVariant, NewTypeDef, TypeHeader};
use object_to_field_shapes::object_to_field_shapes;
use openapi_types::v3_0::{SchemaCase, SchemaFieldName, SchemaObject};
use shape_schema_object_type::shape_schema_object_type;

mod object_to_field_shapes;
mod shape_schema_object_type;
mod shape_type;
mod to_struct;

pub(super) fn from_schema_entry(kv: (SchemaFieldName, SchemaCase)) -> Result<DefinitionShape> {
    let (field_name, schema_case) = kv;
    match schema_case {
        SchemaCase::Schema(obj) => Shaper::run(field_name, *obj),
        SchemaCase::Reference(_) => todo!(),
    }
}

struct Shaper {
    name: SchemaFieldName,
    object: SchemaObject,
}

impl Shaper {
    fn run(name: SchemaFieldName, object: SchemaObject) -> Result<DefinitionShape> {
        Self { name, object }.shape()
    }

    fn shape(self) -> Result<DefinitionShape> {
        if self.object.all_of.is_some() {
            return self.for_all_of();
        }

        use openapi_types::v3_0::OpenApiDataType as ot;
        match self.object.data_type.as_ref() {
            Some(ot::Object) => self.for_struct(),
            Some(ot::String) => match self.object.enum_values {
                Some(_) => self.for_enum(),
                None => self.for_newtype(),
            },
            Some(ot::Integer | ot::Number | ot::Boolean | ot::Array) => self.for_newtype(),

            // define it as 'object' if 'type' is not specified.
            None => self.for_struct(),
        }
    }

    fn for_all_of(self) -> Result<DefinitionShape> {
        let cases = self.object.all_of.unwrap();
        let shapes = cases
            .into_iter()
            .map(to_all_of_item_shape)
            .collect::<Result<Vec<AllOfItemShape>>>()?;

        let doc_comments = Some("TODO: extract doc_comments".to_string());
        let header = TypeHeader::new(self.name, doc_comments);
        let process = PostProcess::AllOf { header, shapes };
        Ok(process.into())
    }

    fn for_newtype(self) -> Result<DefinitionShape> {
        match shape_schema_object_type(self.object)? {
            TypeShape::Fixed(data_type) => {
                let def = NewTypeDef::new(self.name, data_type);
                Ok(Fixed(def.into()))
            }
            type_shape => Ok(InProcess(PostProcess::NewType {
                struct_name: self.name.into(),
                type_shape,
            })),
        }
    }

    fn for_enum(self) -> Result<DefinitionShape> {
        let values = self.object.enum_values.unwrap();
        let variants = values.into_iter().map(EnumVariant::new).collect();
        let def = EnumDef::new(self.name, variants);
        Ok(Fixed(def.into()))
    }
}

fn to_all_of_item_shape(case: SchemaCase) -> Result<AllOfItemShape> {
    let shape = match case {
        SchemaCase::Schema(object) => {
            let object = *object;
            let shapes = object_to_field_shapes(object.properties, object.required)?;
            AllOfItemShape::Object(shapes)
        }
        SchemaCase::Reference(x) => AllOfItemShape::Ref(x),
    };
    Ok(shape)
}
