use crate::targets::rust_type::{Definitions, Imports};

#[derive(Clone, Debug, Default)]
pub struct CoreShape {
    pub imports: Imports,
    pub defs: Definitions,
}
