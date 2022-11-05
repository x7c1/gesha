use crate::render;
use crate::renderer::Result;
use crate::targets::rust_type::{EnumVariantName, MediaTypeDef};
use std::io::Write;

pub fn render_media_type<W: Write>(mut write: W, x: MediaTypeDef) -> Result<()> {
    println!("media_type: {:#?}", x);
    let variants = x
        .translator
        .into_iter()
        .map(|(k, _)| k)
        .collect::<Vec<EnumVariantName>>()
        .join(",");

    render! { write =>
        echo > "#[derive(Clone, Copy, Debug, PartialEq)]";
        echo > "pub enum MediaType {{ {variants} }}";
    }
    Ok(())
}
