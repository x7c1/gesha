use super::{render_enum_variants, render_header};
use crate::{MediaTypeVariant, MediaTypeVariants, RequestBodyDef, render};
use std::fmt;
use std::fmt::Write;

pub fn render_request_body(write: &mut impl Write, x: &RequestBodyDef) -> fmt::Result {
    render! { write =>
        call > render_header => &x.header;
        echo > "pub enum {name}", name = x.header.name;
        "{}" > render_enum_variants => x.variants.iter().map(|x| &x.variant);
        echo > "\n\n";
        echo > "impl {name}", name = x.header.name;
        "{}" > render_impl_body => &x.variants;
    }
    Ok(())
}

fn render_impl_body(write: &mut impl Write, x: &MediaTypeVariants) -> fmt::Result {
    render! { write =>
        echo >
            "pub fn media_type(&self) -> super::core::MediaType {{
                match self {{
                    {arms}
                }}
            }}",
            arms = to_arms_media_type(x);
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

fn to_arms_media_type(xs: &MediaTypeVariants) -> String {
    xs.iter()
        .map(|x| {
            format!(
                "Self::{name}(_) => super::core::MediaType::{name}",
                name = x.variant.name
            )
        })
        .collect::<Vec<String>>()
        .join(",")
}

fn to_arms_new(xs: &MediaTypeVariants) -> String {
    let mut arms = xs.iter().flat_map(create_new_arm).collect::<Vec<String>>();

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

fn create_new_arm(x: &MediaTypeVariant) -> Option<String> {
    match x.header_value.as_str() {
        "application/json" => Some(
            r#"
                "application/json" => {
                    let body = super::core::from_json(value)?;
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
