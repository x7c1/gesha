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
