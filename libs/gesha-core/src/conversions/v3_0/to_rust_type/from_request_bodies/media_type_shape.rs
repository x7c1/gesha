use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum MediaTypeShape {
    ApplicationJson,
    Unsupported(String),
}

impl MediaTypeShape {
    pub fn new<A: Into<String>>(key: A) -> MediaTypeShape {
        let key = key.into();
        match key.as_str() {
            "application/json" => MediaTypeShape::ApplicationJson,
            x => MediaTypeShape::Unsupported(x.into()),
        }
    }
}

impl AsRef<str> for MediaTypeShape {
    fn as_ref(&self) -> &str {
        match self {
            MediaTypeShape::ApplicationJson => "application/json",
            MediaTypeShape::Unsupported(x) => x,
        }
    }
}

impl Display for MediaTypeShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_ref(), f)
    }
}
