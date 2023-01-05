use crate::conversions::v3_0::to_rust_type::components::schemas::{
    to_type_shape, AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, StructShape,
    TypeHeaderShape,
};
use crate::conversions::Result;
use openapi_types::v3_0::{ComponentName, SchemaCase, SchemaObject, SchemasObject};

#[derive(Debug, Clone)]
pub struct SchemasShape(Vec<DefinitionShape>);

impl SchemasShape {
    pub fn from(object: SchemasObject) -> Result<Self> {
        let xs = object.into_iter().map(new).collect::<Result<Vec<_>>>()?;
        Ok(SchemasShape(xs))
    }
    pub fn empty() -> Self {
        Self(vec![])
    }
    pub fn iter(&self) -> impl Iterator<Item = &DefinitionShape> {
        self.0.iter()
    }
}

impl FromIterator<DefinitionShape> for SchemasShape {
    fn from_iter<T: IntoIterator<Item = DefinitionShape>>(iter: T) -> Self {
        let xs = iter.into_iter().collect();
        Self(xs)
    }
}

impl IntoIterator for SchemasShape {
    type Item = <Vec<DefinitionShape> as IntoIterator>::Item;
    type IntoIter = <Vec<DefinitionShape> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

fn new(kv: (ComponentName, SchemaCase)) -> Result<DefinitionShape> {
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
    name: ComponentName,
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

    fn for_struct(self) -> Result<DefinitionShape> {
        let shape = StructShape {
            header: self.create_type_header(),
            fields: FieldShape::from_object(self.object)?,
        };
        Ok(shape.into())
    }

    fn for_all_of(self) -> Result<DefinitionShape> {
        let shape = AllOfShape {
            header: self.create_type_header(),
            items: {
                let cases = self.object.all_of.expect("all_of must be Some.");
                AllOfItemShape::from_schema_cases(cases)?
            },
        };
        Ok(shape.into())
    }

    fn for_newtype(self) -> Result<DefinitionShape> {
        let shape = DefinitionShape::NewType {
            header: self.create_type_header(),
            type_shape: to_type_shape::from_object(self.object, /* is_required */ true)?,
        };
        Ok(shape)
    }

    fn for_enum(self) -> Result<DefinitionShape> {
        let shape = DefinitionShape::Enum {
            header: self.create_type_header(),
            values: self.object.enum_values.expect("enum_values must be Some."),
        };
        Ok(shape)
    }

    fn create_type_header(&self) -> TypeHeaderShape {
        TypeHeaderShape::new(self.name.clone(), &self.object)
    }
}
