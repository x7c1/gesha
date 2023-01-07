use crate::conversions::v3_0::to_rust_type::components::core::CoreShape;
use crate::conversions::Result;
use crate::targets::rust_type::ModDef;

pub fn define_core(shape: CoreShape) -> Result<Option<ModDef>> {
    println!("TODO: {:#?}", shape);
    Ok(None)
}
