use crate::render;
use crate::renderer::Result;
use crate::targets::rust_type::MediaTypeDef;
use std::io::Write;

pub fn render_media_type<W: Write>(mut write: W, x: MediaTypeDef) -> Result<()> {
    println!("media_type: {:#?}", x);
    render! { write =>
        echo > "/* TODO */"
    }
    Ok(())
}
