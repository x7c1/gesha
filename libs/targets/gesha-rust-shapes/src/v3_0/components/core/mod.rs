use gesha_core::conversion::Result;
use gesha_rust_types::{Definitions, Imports, ModDef, ModuleName};
use std::ops::Not;

#[derive(Clone, Debug, Default)]
pub struct CoreShape {
    pub imports: Imports,
    pub defs: Definitions,
}

impl CoreShape {
    pub fn define(self) -> Result<Option<ModDef>> {
        let def = self.defs.is_empty().not().then(|| ModDef {
            name: ModuleName::new("core"),
            imports: self.imports,
            defs: self.defs,
        });
        Ok(def)
    }
}
