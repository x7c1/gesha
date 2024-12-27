use super::{render_data_types, render_header};
use crate::{render, EnumCase, EnumDef, EnumVariant, EnumVariantAttribute};
use std::fmt;
use std::fmt::Write;

pub fn render_enum(write: &mut impl Write, x: &EnumDef) -> fmt::Result {
    render! { write =>
        call > render_header => &x.header;
        echo > "pub enum {name}", name = x.header.name;
        "{}" > render_enum_variants => x.variants.iter();
        echo > "\n\n";
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
        EnumCase::Unit => { /* nop */ }
        EnumCase::Tuple(types) => {
            render! { write =>
                "()" > render_data_types => &types;
            }
        }
    }
    Ok(())
}
