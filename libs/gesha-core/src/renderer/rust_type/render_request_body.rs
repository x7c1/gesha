use crate::render;
use crate::renderer::rust_type::render_derive_attrs;
use crate::renderer::Result;
use crate::targets::rust_type::RequestBodyDef;
use std::io::Write;

pub fn render_request_body<W: Write>(mut write: W, x: RequestBodyDef) -> Result<()> {
    render! { write =>
        echo > "{comments}", comments = x.header.doc_comments;
        call > render_derive_attrs => &x.derive_attrs;
        echo > "#[serde(untagged)]\n";
        echo > "pub enum {name} {{}}", name = x.header.name;
        echo > "\n\n";
    }
    Ok(())
}
