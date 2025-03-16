use super::{render_data_types, render_header};
use crate::{
    render, EnumCase, EnumDef, EnumMacroImpl, EnumMacroType, EnumMacroVariants, EnumVariant,
    EnumVariantAttribute,
};
use indexmap::IndexMap;
use std::fmt;
use std::fmt::Write;

pub fn render_enum(write: &mut impl Write, x: &EnumDef) -> fmt::Result {
    render! { write =>
        call > render_header => &x.header;
        echo > "pub enum {name}", name = x.header.name;
        "{}" > render_enum_variants => x.variants.iter();
        echo > "\n\n";
    }
    if let Some(macro_impl) = &x.macro_impl {
        render! { write =>
            echo > "gesha_macros::impl_enum_serde!";
            "()" > render_enum_macro_impl => macro_impl;
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

fn render_enum_macro_impl(write: &mut impl Write, x: &EnumMacroImpl) -> fmt::Result {
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
