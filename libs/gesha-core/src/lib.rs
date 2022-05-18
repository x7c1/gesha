mod error;
pub use error::{Error, Result};

pub mod targets;

mod reader;
pub use reader::{open_document_file, to_rust_modules};

mod v3_0;
mod yaml_wrapper;

#[derive(Debug)]
pub enum OpenApiDocument {
    V3_0(openapi_types::v3_0::Document),
}
