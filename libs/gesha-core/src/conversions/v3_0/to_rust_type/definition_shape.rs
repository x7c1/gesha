use crate::conversions::v3_0::to_rust_type::{FieldShape, PostProcess};
use crate::targets::rust_type::Definition;
use DefinitionShape::{Fixed, InProcess};

#[derive(Clone, Debug)]
pub(super) enum DefinitionShape {
    Fixed(Definition),
    InProcess(PostProcess),
}

impl DefinitionShape {
    pub fn is_struct_name(&self, name: &str) -> bool {
        match self {
            Fixed(def) => match def {
                Definition::StructDef(x) => x.name == name,
                _ => false,
            },
            InProcess(process) => match process {
                PostProcess::Struct { struct_name, .. } => struct_name == name,
                PostProcess::AllOf { .. } => unimplemented!(),
                PostProcess::NewType { .. } => unimplemented!(),
            },
        }
    }

    pub fn field_shapes(&self) -> Vec<FieldShape> {
        match self {
            Fixed(def) => match def {
                Definition::StructDef(x) => x
                    .fields
                    .clone()
                    .into_iter()
                    .map(FieldShape::Fixed)
                    .collect(),

                _ => vec![],
            },
            InProcess(process) => match process {
                PostProcess::Struct { shapes, .. } => shapes.clone(),
                PostProcess::AllOf { .. } => unimplemented!(),
                PostProcess::NewType { .. } => unimplemented!(),
            },
        }
    }
}