mod to_field_shapes;
use to_field_shapes::to_field_shapes;

mod to_type_shape;
use to_type_shape::to_type_shape;

mod for_struct;

use crate::conversions::v3_0::to_rust_type::DefinitionShape::{Fixed, InProcess};
use crate::conversions::v3_0::to_rust_type::{
    AllOfItemShape, DefinitionShape, PostProcess, TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{DocComments, EnumDef, EnumVariant, NewTypeDef, TypeHeader};
use openapi_types::v3_0::{SchemaCase, SchemaFieldName, SchemaObject};

pub(super) fn to_shape(kv: (SchemaFieldName, SchemaCase)) -> Result<DefinitionShape> {
    let (field_name, schema_case) = kv;
    match schema_case {
        SchemaCase::Schema(obj) => {
            let (name, object) = (field_name, *obj);
            Shaper { name, object }.run()
        }
        SchemaCase::Reference(_) => todo!(),
    }
}

struct Shaper {
    name: SchemaFieldName,
    object: SchemaObject,
}

impl Shaper {
    fn run(self) -> Result<DefinitionShape> {
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
        let header = self.create_type_header();
        let cases = self.object.all_of.expect("all_of must be Some.");
        let shapes = cases
            .into_iter()
            .map(to_all_of_item_shape)
            .collect::<Result<Vec<AllOfItemShape>>>()?;

        let process = PostProcess::AllOf { header, shapes };
        Ok(process.into())
    }

    fn for_newtype(self) -> Result<DefinitionShape> {
        let header = self.create_type_header();
        match to_type_shape::from_object(self.object)? {
            TypeShape::Fixed(data_type) => {
                let def = NewTypeDef::new(header, data_type);
                Ok(Fixed(def.into()))
            }
            type_shape => Ok(InProcess(PostProcess::NewType { header, type_shape })),
        }
    }

    fn for_enum(self) -> Result<DefinitionShape> {
        let header = self.create_type_header();
        let values = self.object.enum_values.expect("enum_values must be Some.");
        let variants = values.into_iter().map(EnumVariant::new).collect();
        let def = EnumDef::new(header, variants);
        Ok(Fixed(def.into()))
    }

    fn create_type_header(&self) -> TypeHeader {
        TypeHeader::new(
            self.name.clone(),
            to_doc_comments(self.object.title.clone(), self.object.description.clone()),
        )
    }
}

fn to_all_of_item_shape(case: SchemaCase) -> Result<AllOfItemShape> {
    let shape = match case {
        SchemaCase::Schema(object) => {
            let object = *object;
            let shapes = to_field_shapes(object.properties, object.required)?;
            AllOfItemShape::Object(shapes)
        }
        SchemaCase::Reference(x) => AllOfItemShape::Ref(x),
    };
    Ok(shape)
}

fn to_doc_comments(title: Option<String>, description: Option<String>) -> DocComments {
    let maybe = match (title, description) {
        (t, None) => t,
        (None, d) => d,
        (t, d) if t == d => t,
        (Some(t), Some(d)) => Some(format!("{t}\n\n{d}")),
    };
    DocComments::new(maybe.map(|x| {
        let text = x.trim().to_string();
        format!("/**\n{text}\n*/\n")
    }))
}
