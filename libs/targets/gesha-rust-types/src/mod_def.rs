use crate::{Definition, Definitions, Imports, ModuleName};

#[derive(Clone, Debug, PartialEq)]
pub struct ModDef {
    pub name: ModuleName,
    pub imports: Imports,
    pub defs: Definitions,
}

impl ModDef {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl From<ModDef> for Definition {
    fn from(this: ModDef) -> Self {
        Self::ModDef(this)
    }
}
