use crate::render;
use crate::renderer::Result;
use gesha_rust_types::{ErrorDef, ErrorVariant};
use std::io::Write;

pub fn render_error<W: Write>(mut write: W, x: ErrorDef) -> Result<()> {
    let variants = x
        .into_iter()
        .map(format_variant)
        .collect::<Vec<&str>>()
        .join(",");

    render! { write =>
        echo > r#"
            pub type Result<A> = std::result::Result<A, Error>;

            #[derive(Debug)]
            pub enum Error {{
                {variants}
            }}
        "#
    }
    Ok(())
}

fn format_variant(x: ErrorVariant) -> &'static str {
    match x {
        ErrorVariant::InvalidJson => "InvalidJson(serde_json::Error)",
        ErrorVariant::UnsupportedMediaType => "UnsupportedMediaType { given: String }",
    }
}
