use crate::conversions::v3_0::to_rust_type::post_process::PostProcessor;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::{Fixed, InProcess};
use crate::conversions::v3_0::to_rust_type::{
    AllOfItemShape, DefinitionShape, FieldShape, PostProcess,
};
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, StructDef, StructField};
use openapi_types::v3_0::ReferenceObject;

impl PostProcessor {
    pub(super) fn resolve_all_of(&self, shape: &mut DefinitionShape) -> Result<()> {
        if let InProcess(process) = shape {
            if let Some(processed) = self.shape_1st_process(process)? {
                *shape = processed;
            }
        } else {
        }
        Ok(())
    }

    fn shape_1st_process(&self, process: &mut PostProcess) -> Result<Option<DefinitionShape>> {
        match process {
            PostProcess::AllOf {
                struct_name,
                shapes,
            } => {
                let def = StructDef {
                    name: struct_name.clone(),
                    fields: self.merge_fields_all_of(shapes)?,
                };
                Ok(Some(Fixed(def.into())))
            }
            PostProcess::Struct { .. } => {
                // shaped in next process.
                Ok(None)
            }
            PostProcess::NewType { .. } => {
                // shaped in next process.
                Ok(None)
            }
        }
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
            FieldShape::InProcess {
                name,
                type_shape,
                is_optional,
            } => {
                unimplemented!("{} {:?} {}", name, type_shape, is_optional)
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
            InProcess(process) => {
                /*
                match process {
                    PostProcess::Struct {
                        struct_name,
                        shapes,
                    } if struct_name == name => {
                        unimplemented!("shapes: {:#?}", shapes)
                    }
                    PostProcess::Struct { .. } => None,
                    PostProcess::NewType { .. } => None,
                    PostProcess::AllOf { .. } => unimplemented!(),
                }
                 */
                unimplemented!("not processed: {:#?}", process)
            }
        });
        xs.unwrap_or_else(|| unimplemented!())
    }
}
