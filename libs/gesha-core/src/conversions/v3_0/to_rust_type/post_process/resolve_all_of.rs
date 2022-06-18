use crate::conversions::v3_0::to_rust_type::post_process::PostProcessor;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::{Fixed, InProcess};
use crate::conversions::v3_0::to_rust_type::{
    AllOfItemShape, DefinitionShape, FieldShape, PostProcess,
};
use crate::conversions::Result;
use crate::targets::rust_type::Definition;
use openapi_types::v3_0::ReferenceObject;

impl PostProcessor {
    pub(super) fn resolve_all_of(&self, shape: &mut DefinitionShape) -> Result<()> {
        if let InProcess(process) = shape {
            if let Some(processed) = self.shape_all_of(process)? {
                *shape = processed;
            }
        } else {
        }
        Ok(())
    }

    fn shape_all_of(&self, process: &mut PostProcess) -> Result<Option<DefinitionShape>> {
        match process {
            PostProcess::AllOf {
                struct_name,
                shapes,
            } => {
                let process = PostProcess::Struct {
                    struct_name: struct_name.clone(),
                    shapes: self.merge_fields_all_of(shapes)?,
                };
                Ok(Some(process.into()))
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

    fn merge_fields_all_of(&self, shapes: &[AllOfItemShape]) -> Result<Vec<FieldShape>> {
        let fields = shapes
            .iter()
            .flat_map(|shape| self.shape_to_fields(shape))
            .collect::<Vec<FieldShape>>();

        Ok(fields)
    }

    fn shape_to_fields(&self, item_shape: &AllOfItemShape) -> Vec<FieldShape> {
        match item_shape {
            AllOfItemShape::Object(shapes) => shapes.clone(),
            AllOfItemShape::Ref(object) => self.find_struct_fields(object),
        }
    }

    fn find_struct_fields(&self, x: &ReferenceObject) -> Vec<FieldShape> {
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

    fn traverse(&self, name: &str, defs: &[DefinitionShape]) -> Vec<FieldShape> {
        defs.iter()
            .find_map(extract_fields(name))
            .unwrap_or_else(|| unimplemented!())
    }
}

fn extract_fields(name: &str) -> impl Fn(&DefinitionShape) -> Option<Vec<FieldShape>> + '_ {
    move |def_shape| match def_shape {
        Fixed(def) => match def {
            Definition::StructDef(x) if x.name == name => Some(
                x.fields
                    .clone()
                    .into_iter()
                    .map(FieldShape::Fixed)
                    .collect(),
            ),
            _ => None,
        },
        InProcess(process) => match process {
            PostProcess::Struct {
                struct_name,
                shapes,
            } if struct_name == name => Some(shapes.clone()),
            PostProcess::Struct { .. } => None,
            PostProcess::NewType { .. } => unimplemented!(),
            PostProcess::AllOf { .. } => unimplemented!(),
        },
    }
}
