use super::{AllOfItemShape, ComponentsShapes, DefinitionShape, FieldShape, PostProcess};
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, StructDef, StructField};
use openapi_types::v3_0::ReferenceObject;
use DefinitionShape::{Fixed, InProcess};

pub(super) struct PostProcessor {
    original: ComponentsShapes,
}

impl PostProcessor {
    pub fn run(modules: &mut ComponentsShapes) -> Result<()> {
        let this = Self {
            original: modules.clone(),
        };
        modules.schemas.iter_mut().try_for_each(|x| this.replace(x))
    }

    fn replace(&self, shape: &mut DefinitionShape) -> Result<()> {
        if let InProcess(process) = shape {
            match process {
                PostProcess::AllOf { name, shapes } => {
                    let def = StructDef {
                        name: name.clone(),
                        fields: self.merge_fields_all_of(shapes)?,
                    };
                    *shape = Fixed(def.into())
                }
            }
        }
        Ok(())
    }

    fn merge_fields_all_of(&self, shapes: &[AllOfItemShape]) -> Result<Vec<StructField>> {
        let fields = shapes
            .iter()
            .flat_map(|shape| self.shape_to_fields(shape))
            .collect::<Vec<StructField>>();

        Ok(fields)
    }

    fn shape_to_fields(&self, item_shape: &AllOfItemShape) -> Vec<StructField> {
        let to_field = |shape: &FieldShape| match shape {
            FieldShape::Fixed(field) => field.clone(),
            FieldShape::InProcess { name, type_shape } => {
                unimplemented!("{} {:?}", name, type_shape)
            }
        };
        match item_shape {
            AllOfItemShape::Object(shapes) => shapes.iter().map(to_field).collect(),
            AllOfItemShape::Ref(object) => self.find_struct_fields(object),
        }
    }

    fn find_struct_fields(&self, x: &ReferenceObject) -> Vec<StructField> {
        // TODO: support locations other than 'schemas'
        let prefix = "#/components/schemas/";
        let type_ref = x.as_ref();
        if type_ref.starts_with(prefix) {
            let name = type_ref.replace(prefix, "");
            self.traverse(&name, &self.original.schemas)
        } else {
            unimplemented!()
        }
    }

    fn traverse(&self, name: &str, defs: &[DefinitionShape]) -> Vec<StructField> {
        let xs = defs.iter().find_map(|def_shape| match def_shape {
            Fixed(def) => match def {
                Definition::StructDef(x) if x.name == name => Some(x.fields.clone()),
                _ => None,
            },
            InProcess(_) => unimplemented!(),
        });
        xs.unwrap_or_else(|| unimplemented!())
    }
}
