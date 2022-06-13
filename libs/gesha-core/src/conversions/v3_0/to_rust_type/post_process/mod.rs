use super::{DefinitionShape, PostProcess};
use crate::conversions::v3_0::to_rust_type::DefinitionShape::Fixed;
use crate::conversions::v3_0::to_rust_type::{AllOfItemShape, ComponentsShapes, FieldShape};
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, StructDef, StructField};
use openapi_types::v3_0::ReferenceObject;
use DefinitionShape::InProcess;

pub(super) fn post_process(modules: &mut ComponentsShapes) -> Result<()> {
    let processor = Processor {
        original: Clone::clone(modules),
    };
    processor.run(modules)
}

struct Processor {
    original: ComponentsShapes,
}

impl Processor {
    fn run(self, modules: &mut ComponentsShapes) -> Result<()> {
        modules
            .schemas
            .iter_mut()
            .try_for_each(|x| self.replace(x))?;

        Ok(())
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
            .flat_map(|shape| match shape {
                AllOfItemShape::Object(x) => x
                    .iter()
                    .map(|field_shape| match field_shape {
                        FieldShape::Fixed(field) => field.clone(),
                        FieldShape::InProcess { name, type_shape } => {
                            unimplemented!("{} {:?}", name, type_shape)
                        }
                    })
                    .collect::<Vec<StructField>>(),
                AllOfItemShape::Ref(x) => self.find_struct_fields(x),
            })
            .collect::<Vec<StructField>>();

        Ok(fields)
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
