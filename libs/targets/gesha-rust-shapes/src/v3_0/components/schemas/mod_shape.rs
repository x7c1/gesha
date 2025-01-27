use crate::misc::TryMap;
use crate::v3_0::components::schemas::{DefinitionShape, TypeShape};
use gesha_core::conversions::Result;
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

    pub fn any_type(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        self.defs.iter().any(|x| x.any_type(f))
    }

    pub fn any_type_directly(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        self.defs.iter().any(|x| x.any_type_directly(f))
    }

    pub fn map_def(
        mut self,
        f: impl Fn(DefinitionShape) -> Result<DefinitionShape>,
    ) -> Result<Self> {
        self.defs = self.defs.try_map(f)?;
        Ok(self)
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

impl From<ModShape> for DefinitionShape {
    fn from(this: ModShape) -> Self {
        DefinitionShape::Mod(this)
    }
}
