use super::{Fragment, PostProcess};
use crate::conversions::v3_0::to_rust_type::ComponentFragments;
use crate::conversions::v3_0::to_rust_type::Fragment::Fixed;
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, StructDef, StructField};
use openapi_types::v3_0::{AllOf, SchemaCase};

pub(super) fn post_process(modules: &mut ComponentFragments) -> Result<()> {
    let processor = Processor {
        original: Clone::clone(modules),
    };
    processor.run(modules)
}

struct Processor {
    original: ComponentFragments,
}

impl Processor {
    fn run(self, modules: &mut ComponentFragments) -> Result<()> {
        modules
            .schemas
            .iter_mut()
            .try_for_each(|x| self.replace(x))?;

        Ok(())
    }

    fn replace(&self, fragment: &mut Fragment) -> Result<()> {
        if let Fragment::InProcess(process) = fragment {
            match process {
                PostProcess::AllOf { name: name0, cases } => {
                    *fragment = Fixed(Definition::StructDef(StructDef {
                        name: name0.clone(),
                        fields: self.merge_fields_all_of(cases)?,
                    }))
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
