use crate::{EnumMacroSerdeImpl, EnumMacroType, EnumMacroVariants, render};
use indexmap::IndexMap;
use std::fmt;
use std::fmt::Write;

pub fn render_macro_for_serde(write: &mut impl Write, x: &EnumMacroSerdeImpl) -> fmt::Result {
    render! { write =>
        echo > "{name}", name = x.name;
        "{}" > render_enum_macro_type_variants => &x.type_variants;
    }
    Ok(())
}

fn render_enum_macro_type_variants(
    write: &mut impl Write,
    type_variants: &IndexMap<EnumMacroType, EnumMacroVariants>,
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

fn render_enum_macro_variants(write: &mut impl Write, variants: &EnumMacroVariants) -> fmt::Result {
    let pairs = variants
        .iter()
        .map(|(name, constant)| format!("({name}, {constant})"))
        .collect::<Vec<_>>()
        .join(",");

    render! { write =>
        echo > "{pairs}";
    }
    Ok(())
}
