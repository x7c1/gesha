use crate::conversions::v3_0::to_rust_type::components::request_bodies::DefinitionShape;
use crate::conversions::Result;
use crate::targets::rust_type::{ModDef, ModuleName, Package};
use openapi_types::v3_0::ComponentName;

#[derive(Clone, Debug)]
pub struct ModShape {
    pub imports: Vec<Package>,
    pub name: ComponentName,
    pub defs: Vec<DefinitionShape>,
}

impl ModShape {
    pub fn new(name: ComponentName, defs: Vec<DefinitionShape>) -> Self {
        Self {
            name,
            defs,
            imports: vec![Package::Deserialize, Package::Serialize],
        }
    }

    pub fn define(self) -> Result<ModDef> {
        let defs = self
            .defs
            .into_iter()
            .map(|x| x.define())
            .collect::<Result<Vec<_>>>()?;

        let def = ModDef {
            name: ModuleName::new(self.name),
            imports: self.imports.into(),
            defs: defs.into_iter().collect(),
        };
        Ok(def)
    }
}
