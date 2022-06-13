use crate::conversions::v3_0::to_rust_type::DefinitionShape;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::{Fixed, InProcess};
use crate::conversions::Error::RequirePostProcess;
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, ModuleName, Modules};
use indexmap::indexmap;

#[derive(Clone, Debug)]
pub struct ComponentsShapes {
    pub(super) schemas: Vec<DefinitionShape>,
}

impl ComponentsShapes {
    pub fn into_modules(self) -> Result<Modules> {
        let schemas = self
            .schemas
            .into_iter()
            .map(|x| match x {
                Fixed(def) => Ok(def),
                InProcess(process) => Err(RequirePostProcess {
                    detail: format!("{:#?}", process),
                }),
            })
            .collect::<Result<Vec<Definition>>>()?;

        Ok(indexmap! {
             ModuleName::new("schemas") => schemas,
        })
    }
}
