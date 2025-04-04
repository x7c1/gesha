use crate::MediaTypeDef;
use crate::render;
use std::fmt;
use std::fmt::Write;

pub fn render_media_type(mut write: impl Write, x: &MediaTypeDef) -> fmt::Result {
    let variants = x
        .translator
        .iter()
        .map(|(k, _)| k.as_str())
        .collect::<Vec<&str>>()
        .join(",");

    let arms = x
        .translator
        .iter()
        .map(|(k, v)| format!(r#"MediaType::{k} => "{v}" "#))
        .collect::<Vec<String>>()
        .join(",");

    render! { write =>
        echo > "
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub enum MediaType {{ {variants} }}

            impl Display for MediaType {{
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{
                    Display::fmt(self.as_ref(), f)
                }}
            }}

            impl AsRef<str> for MediaType {{
                fn as_ref(&self) -> &str {{
                    match self {{
                        {arms}
                    }}
                }}
            }}
        ";
    }
    Ok(())
}
