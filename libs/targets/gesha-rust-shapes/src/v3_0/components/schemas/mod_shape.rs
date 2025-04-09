use crate::v3_0::components::schemas::{DefinitionShape, EnumShape, TypeShape};
use gesha_collections::seq::TryMapOps;
use gesha_core::conversions::Result;
use gesha_rust_types::{Definitions, DeriveAttribute, ModDef, ModuleName, Package};

#[derive(Clone, Debug)]
pub struct ModShape {
    pub imports: Vec<Package>,
    pub name: ModuleName,
    pub defs: Vec<DefinitionShape>,
}

impl ModShape {
    pub fn new(name: ModuleName, defs: Vec<DefinitionShape>) -> Self {
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

    pub fn any_enum(&self, f: &impl Fn(&EnumShape) -> bool) -> bool {
        self.defs.iter().any(|x| x.any_enum(f))
    }

    pub fn any_derive_directly(&self, mut f: impl FnMut(&DeriveAttribute) -> bool) -> bool {
        self.defs.iter().any(|x| x.any_derive_directly(&mut f))
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
