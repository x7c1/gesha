use super::{DefinitionShape, PostProcess};
use crate::conversions::v3_0::to_rust_type::ComponentShapes;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::Fixed;
use crate::conversions::Result;
use crate::targets::rust_type::{StructDef, StructField};
use openapi_types::v3_0::{AllOf, SchemaCase};
use DefinitionShape::InProcess;

pub(super) fn post_process(modules: &mut ComponentShapes) -> Result<()> {
    let processor = Processor {
        original: Clone::clone(modules),
    };
    processor.run(modules)
}

struct Processor {
    original: ComponentShapes,
}

impl Processor {
    fn run(self, modules: &mut ComponentShapes) -> Result<()> {
        modules
            .schemas
            .iter_mut()
            .try_for_each(|x| self.replace(x))?;

        Ok(())
    }

    fn replace(&self, shape: &mut DefinitionShape) -> Result<()> {
        if let InProcess(process) = shape {
            match process {
                PostProcess::AllOf { name, cases } => {
                    let def = StructDef {
                        name: name.clone(),
                        fields: self.merge_fields_all_of(cases)?,
                    };
                    *shape = Fixed(def.into())
                }
            }
        }
        Ok(())
    }

    fn merge_fields_all_of(&self, cases: &AllOf) -> Result<Vec<StructField>> {
        println!("original: {:#?}", self.original);

        // TODO:
        for case in cases {
            match case {
                SchemaCase::Schema(_) => {}
                SchemaCase::Reference(_) => {}
            }
        }
        Ok(vec![])
    }
}
