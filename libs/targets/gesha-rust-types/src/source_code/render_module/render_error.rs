use crate::{render, ErrorDef, ErrorVariant};
use std::fmt;
use std::fmt::Write;

pub fn render_error(write: &mut impl Write, x: &ErrorDef) -> fmt::Result {
    let variants = x
        .iter()
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

fn format_variant(x: &ErrorVariant) -> &'static str {
    match x {
        ErrorVariant::InvalidJson => "InvalidJson(serde_json::Error)",
        ErrorVariant::UnsupportedMediaType => "UnsupportedMediaType { given: String }",
    }
}
