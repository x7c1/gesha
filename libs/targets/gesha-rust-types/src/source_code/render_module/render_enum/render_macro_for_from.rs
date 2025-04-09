use super::render_enum_macro_variants;
use crate::{EnumMacroForFrom, EnumMacroTypeForFrom, render};
use std::fmt;
use std::fmt::Write;

pub fn render_macro_for_from(write: &mut impl Write, x: &EnumMacroForFrom) -> fmt::Result {
    let prefix = ["super"].repeat(x.depth).join("::");

    render! { write =>
        echo > "{name},", name = x.name;
        echo > "{prefix}::core::Error,";
        "[]" > render_types => &x.types;
        echo > ",";
        "[]" > render_enum_macro_variants => &x.variants;
        echo > ",";
    }
    Ok(())
}

fn render_types(write: &mut impl Write, x: &[EnumMacroTypeForFrom]) -> fmt::Result {
    let types = x
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    render! { write =>
        echo > "{types}";
    }
    Ok(())
}
