use super::render_enum_macro_variants;
use crate::{EnumMacroForFrom, EnumMacroTypeForFrom, render};
use std::fmt;
use std::fmt::Write;

pub fn render_macro_for_from(write: &mut impl Write, x: &EnumMacroForFrom) -> fmt::Result {
    render! { write =>
        echo > "gesha_macros::impl_enum!";
        "()" > render_body => x;
        echo > ";";
        echo > "\n\n";
    }

    Ok(())
}

fn render_body(write: &mut impl Write, x: &EnumMacroForFrom) -> fmt::Result {
    let prefix = ["super"].repeat(x.depth).join("::");
    render! { write =>
        echo > "impl From<{name}>,", name = x.name;
        echo > "impl TryFrom";
        "<>" > render_types => &x.types;
        echo > ",";
        echo > "{prefix}::core::Error,";
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
