use crate::render;
use crate::renderer::rust_type::{render_data_types, render_header};
use crate::renderer::Result;
use gesha_rust_types::{EnumCase, EnumDef, EnumVariant, EnumVariantAttribute};
use std::io::Write;

pub fn render_enum(write: &mut impl Write, x: EnumDef) -> Result<()> {
    render! { write =>
        call > render_header => &x.header;
        echo > "pub enum {name}", name = x.header.name;
        "{}" > render_enum_variants => x.variants;
        echo > "\n\n";
    }
    Ok(())
}

pub fn render_enum_variants<A>(write: &mut impl Write, variants: A) -> Result<()>
where
    A: Into<Vec<EnumVariant>>,
{
    for variant in variants.into() {
        render! { write =>
            call > render_variant_attrs => variant.attributes;
            echo > "{name}", name = variant.name;
            call > render_enum_case => variant.case;
            echo > ",\n";
        }
    }
    Ok(())
}

fn render_variant_attrs(write: &mut impl Write, attrs: Vec<EnumVariantAttribute>) -> Result<()> {
    for attr in attrs.into_iter() {
        render! { write => echo > "#[{attr}]"; }
    }
    Ok(())
}

fn render_enum_case(write: &mut impl Write, case: EnumCase) -> Result<()> {
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
