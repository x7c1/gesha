use super::render_enum_macro_variants;
use crate::{EnumMacroForSerde, EnumMacroTypeForSerde, EnumMacroVariants, render};
use indexmap::IndexMap;
use std::fmt;
use std::fmt::Write;

pub fn render_macro_for_serde(write: &mut impl Write, x: &EnumMacroForSerde) -> fmt::Result {
    render! { write =>
        echo > "{name}", name = x.name;
        "{}" > render_enum_macro_type_variants => &x.type_variants;
    }
    Ok(())
}

fn render_enum_macro_type_variants(
    write: &mut impl Write,
    type_variants: &IndexMap<EnumMacroTypeForSerde, EnumMacroVariants>,
) -> fmt::Result {
    for (name, variants) in type_variants {
        render! { write =>
            echo > "{name}:", name = name.to_string();
            "[]" > render_enum_macro_variants => variants;
            echo > ",";
        }
    }
    Ok(())
}
