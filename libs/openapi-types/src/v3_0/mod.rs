/// OpenAPI Document
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schema
#[derive(Debug)]
pub struct Document {
    pub openapi: String,
    pub info: InfoObject,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#infoObject
#[derive(Debug)]
pub struct InfoObject {
    pub title: String,
}
