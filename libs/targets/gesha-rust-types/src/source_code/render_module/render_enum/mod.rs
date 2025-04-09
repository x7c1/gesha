mod render_macro_for_from;
use render_macro_for_from::render_macro_for_from;

mod render_macro_for_serde;
use render_macro_for_serde::render_macro_for_serde;

use super::{render_data_types, render_header};
use crate::{EnumCase, EnumDef, EnumMacroVariants, EnumVariant, EnumVariantAttribute, render};
use std::fmt;
use std::fmt::Write;

pub fn render_enum(write: &mut impl Write, x: &EnumDef) -> fmt::Result {
    render! { write =>
        call > render_header => &x.header;
        echo > "pub enum {name}", name = x.header.name;
        "{}" > render_enum_variants => x.variants.iter();
        echo > "\n\n";
    }
    if let Some(macro_impl) = &x.macro_for_serde {
        render! { write =>
            echo > "gesha_macros::impl_enum_serde!";
            "()" > render_macro_for_serde => macro_impl;
            echo > ";";
            echo > "\n\n";
        }
    }
    if let Some(macro_impl) = &x.macro_for_from {
        render! { write =>
            echo > "gesha_macros::impl_enum_from!";
            "()" > render_macro_for_from => macro_impl;
            echo > ";";
            echo > "\n\n";
        }
    }
    Ok(())
}

pub fn render_enum_variants<'a>(
    write: &mut impl Write,
    variants: impl Iterator<Item = &'a EnumVariant>,
) -> fmt::Result {
    for variant in variants {
        render! { write =>
            call > render_variant_attrs => &variant.attributes;
            echo > "{name}", name = variant.name;
            call > render_enum_case => &variant.case;
            echo > ",\n";
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

fn render_variant_attrs(write: &mut impl Write, attrs: &[EnumVariantAttribute]) -> fmt::Result {
    for attr in attrs {
        render! { write => echo > "#[{attr}]"; }
    }
    Ok(())
}

fn render_enum_case(write: &mut impl Write, case: &EnumCase) -> fmt::Result {
    match case {
        EnumCase::Unit(_) => { /* nop */ }
        EnumCase::Tuple(types) => {
            render! { write =>
                "()" > render_data_types => &types;
            }
        }
    }
    Ok(())
}
