use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum RequestError {
    QueryStringBroken(String),
    InvalidQueryValue { key: String, message: String },
    InvalidPathValue { key: String, message: String },
    EmptyPathValue { key: String },
}
