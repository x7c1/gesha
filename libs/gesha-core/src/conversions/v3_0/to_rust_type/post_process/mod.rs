use crate::conversions::Result;
use crate::targets::rust_type::{
    Definition, ModuleName, Modules, PostProcess, StructDef, StructField,
};
use openapi_types::v3_0::{AllOf, SchemaCase};

pub fn post_process_components(modules: &mut Modules) -> Result<()> {
    let processor = Processor {
        original: Clone::clone(modules),
    };
    processor.run(modules)
}

struct Processor {
    original: Modules,
}

impl Processor {
    fn run(self, modules: &mut Modules) -> Result<()> {
        if let Some(xs) = modules.get_mut(&ModuleName::new("schemas")) {
            xs.iter_mut().try_for_each(|x| self.replace(x))?;
        }
        Ok(())
    }

    fn replace(&self, def: &mut Definition) -> Result<()> {
        if let Definition::NeedPostProcess(process) = def {
            match process {
                PostProcess::AllOf { name: name0, cases } => {
                    *def = Definition::StructDef(StructDef {
                        name: name0.clone(),
                        fields: self.merge_fields_all_of(cases)?,
                    })
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
