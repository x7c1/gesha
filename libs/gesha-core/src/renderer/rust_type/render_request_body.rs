use crate::render;
use crate::renderer::rust_type::{render_derive_attrs, render_enum_variants};
use crate::renderer::Result;
use crate::targets::rust_type::{MediaTypeVariant, MediaTypeVariants, RequestBodyDef};
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

fn render_impl_body<W: Write>(mut write: W, x: MediaTypeVariants) -> Result<()> {
    render! { write =>
        echo >
            "pub fn media_type(&self) -> super::core::MediaType {{
                match self {{
                    {arms}
                }}
            }}",
            arms = to_arms_media_type(x.clone());
        echo >
            "pub fn new(value: &str, media_type: &str) -> super::core::Result<Self> {{
                match media_type {{
                    {arms}
                }}
            }}",
            arms = to_arms_new(x);
    }
    Ok(())
}

fn to_arms_media_type(xs: MediaTypeVariants) -> String {
    xs.into_iter()
        .map(|x| {
            format!(
                "Self::{name}(_) => super::core::MediaType::{name}",
                name = x.variant.name
            )
        })
        .collect::<Vec<String>>()
        .join(",")
}

fn to_arms_new(xs: MediaTypeVariants) -> String {
    let mut arms = xs
        .into_iter()
        .map(create_new_arm)
        .flatten()
        .collect::<Vec<String>>();

    arms.push(
        r#"
            unsupported => Err(super::core::Error::UnsupportedMediaType {
                given: unsupported.to_string(),
            }),
        "#
        .to_string(),
    );
    arms.join(",")
}

fn create_new_arm(x: MediaTypeVariant) -> Option<String> {
    match x.header_value.as_str() {
        "application/json" => Some(
            r#"
                "application/json" => {
                    let body = super::core::from_json_string(value)?;
                    Ok(Self::ApplicationJson(body))
                }
            "#
            .to_string(),
        ),
        _ => {
            // ignore unsupported media type
            None
        }
    }
}
