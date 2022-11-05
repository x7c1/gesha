use crate::render;
use crate::renderer::Result;
use crate::targets::rust_type::MediaTypeDef;
use std::io::Write;

pub fn render_media_type<W: Write>(mut write: W, x: MediaTypeDef) -> Result<()> {
    let variants = x
        .translator
        .iter()
        .map(|(k, _)| k.as_str())
        .collect::<Vec<&str>>()
        .join(",");

    let arms = x
        .translator
        .iter()
        .map(|(k, &v)| format!(r#"MediaType::{k} => "{v}" "#))
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
            }}";
    }
    Ok(())
}
