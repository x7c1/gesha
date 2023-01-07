use crate::conversions::v3_0::to_rust_type::components::core::CoreShape;
use crate::conversions::Result;
use crate::targets::rust_type::{ModDef, ModuleName};
use std::ops::Not;

pub fn define_core(shape: CoreShape) -> Result<Option<ModDef>> {
    let def = shape.defs.is_empty().not().then(|| ModDef {
        name: ModuleName::new("core"),
        imports: shape.imports,
        defs: shape.defs,
    });
    Ok(def)
}
