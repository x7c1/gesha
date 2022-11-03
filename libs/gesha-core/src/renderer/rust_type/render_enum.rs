use crate::render;
use crate::renderer::rust_type::{render_data_types, render_derive_attrs};
use crate::renderer::Result;
use crate::targets::rust_type::{EnumCase, EnumDef, EnumVariant, EnumVariantAttribute};
use std::io::Write;

pub fn render_enum<W: Write>(mut write: W, x: EnumDef) -> Result<()> {
    render! { write =>
        echo > "{comments}", comments = x.header.doc_comments;
        call > render_derive_attrs => &x.derive_attrs;
        echo > "pub enum {name}", name = x.header.name;
        "{}" > render_enum_variants => x.variants;
        echo > "\n\n";
    }
    Ok(())
}

pub fn render_enum_variants<W, A>(mut write: W, variants: A) -> Result<()>
where
    W: Write,
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

fn render_variant_attrs<W: Write>(mut write: W, attrs: Vec<EnumVariantAttribute>) -> Result<()> {
    for attr in attrs.into_iter() {
        render! { write => echo > "#[{attr}]"; }
    }
    Ok(())
}

fn render_enum_case<W: Write>(mut write: W, case: EnumCase) -> Result<()> {
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
