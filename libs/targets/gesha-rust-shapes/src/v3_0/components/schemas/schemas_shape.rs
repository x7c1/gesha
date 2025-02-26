use crate::v3_0::components::schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, EnumShape, FieldShape, ModShape, NewTypeShape,
    OneOfItemShape, OneOfShape, Ref, StructShape, TypeHeaderShape, TypeShape,
};
use gesha_core::conversions::{by_key, with_key, Result};
use gesha_rust_types::ModDef;
use openapi_types::core::OutputMergeOps;
use openapi_types::v3_0::{ComponentName, SchemaCase, SchemaObject, SchemasObject};
use std::ops::Not;

#[derive(Debug, Clone)]
pub struct SchemasShape {
    pub root: ModShape,
}

impl SchemasShape {
    pub fn shape(maybe: Option<SchemasObject>) -> Result<Self> {
        let mut this = Self {
            root: ModShape::new(ComponentName::new("schemas"), vec![]),
        };
        if let Some(object) = maybe {
            this.root.defs = object
                .into_iter()
                .map(new)
                .collect::<Vec<Result<_>>>()
                .merge()
                .bind_errors(with_key("schemas"))
                .to_result()?;
        }
        Ok(this)
    }

    pub fn define(self) -> Result<Option<ModDef>> {
        let schemas = self.root.define()?;
        Ok(schemas.defs.is_empty().not().then_some(schemas))
    }

    pub fn any_type(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        self.root.defs.iter().any(|x| x.any_type(f))
    }

    pub fn find_type_name(&self, target: &Ref) -> Option<&ComponentName> {
        self.find_header(target).map(|x| &x.name)
    }

    pub fn is_nullable(&self, target: &Ref) -> bool {
        self.find_header(target)
            .map(|x| x.is_nullable)
            .unwrap_or(false)
    }

    pub fn collect_fields(&self, target: &Ref) -> Vec<FieldShape> {
        let name = extract_ref_name(target);
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

    fn find_header(&self, target: &Ref) -> Option<&TypeHeaderShape> {
        let name = extract_ref_name(target);
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
            Shaper { name, object }.run().map_err(by_key(field_name))
        }
        SchemaCase::Reference(_) => todo!(),
    }
}

fn extract_ref_name(target: &Ref) -> &str {
    if let Some(x) = target.as_ref().strip_prefix("#/components/schemas/") {
        x
    } else {
        unimplemented!()
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
        if self.object.one_of.is_some() {
            return self.for_one_of();
        }
        use openapi_types::v3_0::OpenApiDataType as o;
        match self.object.data_type.as_ref() {
            Some(o::Object) => self.for_struct(),
            Some(o::String) => match self.object.enum_values {
                Some(_) => self.for_enum(),
                None => self.for_newtype(),
            },
            Some(o::Integer | o::Number | o::Boolean | o::Array) => self.for_newtype(),

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
                AllOfItemShape::from_schema_cases(cases).to_result()?
            },
        };
        Ok(shape.into())
    }

    fn for_one_of(self) -> Result<DefinitionShape> {
        let shape = OneOfShape {
            header: self.create_type_header(),
            items: {
                let cases = self.object.one_of.expect("one_of must be Some.");
                OneOfItemShape::from_schema_cases(cases)?
            },
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

    fn for_enum(self) -> Result<DefinitionShape> {
        let shape = EnumShape::new(
            self.create_type_header(),
            self.object.enum_values.expect("enum_values must be Some."),
        );
        Ok(shape.into())
    }

    fn create_type_header(&self) -> TypeHeaderShape {
        TypeHeaderShape::new(self.name.clone(), &self.object, vec![])
    }
}
