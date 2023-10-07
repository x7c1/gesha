use crate::conversions::v3_0::to_rust_type::components::request_bodies::DefinitionShape;
use crate::conversions::Result;
use gesha_rust_types::{Definitions, ModDef, ModuleName, Package};
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
        let def = ModDef {
            name: ModuleName::new(self.name),
            imports: self.imports.into(),
            defs: Definitions::from(self.defs)?,
        };
        Ok(def)
    }
}
