use crate::conversions::v3_0::to_rust_type::components::schemas::{DefinitionShape, TypeShape};
use crate::conversions::Result;
use crate::targets::rust_type::Package;
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

    pub fn map_defs(
        mut self,
        f: impl Fn(DefinitionShape) -> Result<DefinitionShape>,
    ) -> Result<Self> {
        self.defs = self.defs.into_iter().map(f).collect::<Result<Vec<_>>>()?;
        Ok(self)
    }
}

impl From<ModShape> for DefinitionShape {
    fn from(this: ModShape) -> Self {
        DefinitionShape::Mod(this)
    }
}
