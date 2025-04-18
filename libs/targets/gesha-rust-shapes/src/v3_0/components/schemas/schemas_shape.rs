use crate::v3_0::components::schemas::{
    AllOfItemShapes, AllOfShape, DefinitionShape, EnumShape, FieldShape, ModShape, NewTypeShape,
    OneOfItemShapes, OneOfShape, RefShape, StructShape, TypeHeaderShape, TypeShape,
};
use gesha_collections::partial_result::MergeOps;
use gesha_collections::tracking::WithContextOps;
use gesha_core::conversions::{Output, Result};
use gesha_rust_types::{ModDef, ModuleName, TypeIdentifier};
use openapi_types::v3_0::{ComponentName, EnumValues, SchemaCase, SchemaObject, SchemasObject};
use std::ops::Not;

#[derive(Debug, Clone)]
pub struct SchemasShape {
    pub root: ModShape,
}

impl SchemasShape {
    pub fn shape(maybe: Option<SchemasObject>) -> Output<Self> {
        let (defs, errors) = if let Some(object) = maybe {
            object
                .into_iter()
                .map(new)
                .collect::<Vec<Result<_>>>()
                .merge()
                .into_tuple()
        } else {
            Default::default()
        };
        let this = Self {
            root: ModShape::new(ModuleName::new("schemas"), defs),
        };
        Output::new(this, errors)
    }

    pub fn define(self) -> Result<Option<ModDef>> {
        let schemas = self.root.define()?;
        Ok(schemas.defs.is_empty().not().then_some(schemas))
    }

    pub fn any_type(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        self.root.defs.iter().any(|x| x.any_type(f))
    }

    pub fn any_enum(&self, f: &impl Fn(&EnumShape) -> bool) -> bool {
        self.root.defs.iter().any(|x| x.any_enum(f))
    }

    pub fn find_type_name(&self, target: &RefShape) -> Option<&TypeIdentifier> {
        self.find_header(target).map(|x| &x.name)
    }

    pub fn is_nullable(&self, target: &RefShape) -> bool {
        self.find_header(target)
            .map(|x| x.is_nullable)
            .unwrap_or(false)
    }

    pub fn collect_fields(&self, target: &RefShape) -> Vec<FieldShape> {
        let name = &target.type_name;
        self.root
            .defs
            .iter()
            .find(|def| {
                def.type_header()
                    .map(|x| x.name.as_ref() == name)
                    .unwrap_or(false)
            })
            .map(|def| def.collect_fields(|x| self.collect_fields(x)))
            .unwrap_or_default()
    }

    fn find_header(&self, target: &RefShape) -> Option<&TypeHeaderShape> {
        let name = &target.type_name;
        self.root
            .defs
            .iter()
            .flat_map(|x| x.type_header())
            .find(|x| x.name.as_ref() == name)
    }
}

fn new(kv: (ComponentName, SchemaCase)) -> Result<DefinitionShape> {
    let (field_name, schema_case) = kv;
    match schema_case {
        SchemaCase::Schema(obj) => {
            let (name, object) = (field_name.clone(), *obj);
            let name = TypeIdentifier::parse(&name)?;
            Shaper { name, object }.run().with_context(field_name)
        }
        SchemaCase::Reference(obj) => {
            let type_shape = RefShape::new(obj, /* is_required */ true)?;
            let type_name = TypeIdentifier::parse(&field_name)?;
            let header = TypeHeaderShape::from_name(type_name);
            let shape = NewTypeShape::new(header, type_shape.into());
            Ok(shape.into())
        }
    }
}

struct Shaper {
    name: TypeIdentifier,
    object: SchemaObject,
}

impl Shaper {
    fn run(self) -> Result<DefinitionShape> {
        if self.object.all_of.is_some() {
            return self.for_all_of();
        }
        if self.object.one_of.is_some() {
            return self.for_one_of();
        }
        if let Some(values) = self.object.enum_values.clone() {
            return self.for_enum(values);
        }
        use openapi_types::v3_0::OpenApiDataType as o;
        match self.object.data_type.as_ref() {
            Some(o::Object) => self.for_struct(),
            Some(o::String | o::Integer | o::Boolean | o::Number | o::Array) => self.for_newtype(),

            // define it as 'object' if 'type' is not specified.
            None => self.for_struct(),
        }
    }

    fn for_struct(self) -> Result<DefinitionShape> {
        let shape = StructShape {
            header: self.create_type_header(),
            fields: FieldShape::from_object(self.object).to_result()?,
        };
        Ok(shape.into())
    }

    fn for_all_of(self) -> Result<DefinitionShape> {
        let shape = AllOfShape {
            header: self.create_type_header(),
            required: self.object.required,
            items: {
                let cases = self.object.all_of.expect("all_of must be Some.");
                AllOfItemShapes::from_schema_cases(cases).to_result()?
            },
        };
        Ok(shape.into())
    }

    fn for_one_of(self) -> Result<DefinitionShape> {
        let shape = OneOfShape {
            header: self.create_type_header(),
            items: {
                let cases = self.object.one_of.expect("one_of must be Some.");
                OneOfItemShapes::from_schema_cases(cases).to_result()?
            },
            format: self.object.format,
        };
        Ok(shape.into())
    }

    fn for_newtype(self) -> Result<DefinitionShape> {
        let shape = NewTypeShape::new(
            self.create_type_header(),
            TypeShape::from_object(self.object, /* is_required */ true)?,
        );
        Ok(shape.into())
    }

    fn for_enum(self, values: EnumValues) -> Result<DefinitionShape> {
        let shape = EnumShape::new(self.create_type_header(), values, self.object.format)?;
        Ok(shape.into())
    }

    fn create_type_header(&self) -> TypeHeaderShape {
        TypeHeaderShape::new(self.name.clone(), &self.object, vec![])
    }
}
