use crate::render;
use crate::renderer::rust_type::{render_derive_attrs, render_enum_variants};
use crate::renderer::Result;
use crate::targets::rust_type::{MediaTypeVariants, RequestBodyDef};
use std::io::Write;

pub fn render_request_body<W: Write>(mut write: W, x: RequestBodyDef) -> Result<()> {
    render! { write =>
        echo > "{comments}", comments = x.header.doc_comments;
        call > render_derive_attrs => &x.derive_attrs;
        echo > "#[serde(untagged)]\n";
        echo > "pub enum {name}", name = x.header.name;
        "{}" > render_enum_variants => x.variants.clone();
        echo > "\n\n";
        echo > "impl {name}", name = x.header.name;
        "{}" > render_impl_body => x.variants;
    }
    Ok(())
}

// fn render_impl<W: Write>(mut write: W, x: RequestBodyDef) -> Result<()> {
//     println!(".......{:#?}", x.variants);
//     render! { write =>
//         echo > "impl {name}", name = x.header.name;
//         "{}" > render_impl_body => x.variants;
//     }
//     Ok(())
// }

fn render_impl_body<W: Write>(mut write: W, x: MediaTypeVariants) -> Result<()> {
    render! { write =>
        echo >
            "pub fn media_type(&self) -> super::core::MediaType {{
                match self {{
                    {arms}
                }}
            }}",
            arms = to_arms(x);
    }
    Ok(())
}

fn to_arms(x: MediaTypeVariants) -> String {
    // TODO:
    // generate string like below
    // "Self::ApplicationJson(_) => super::core::MediaType::ApplicationJson",
    "sample".to_string()
}
