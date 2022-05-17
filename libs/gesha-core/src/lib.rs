mod error;
pub use error::{Error, Result};

pub mod rust_types;

mod reader;
pub use reader::{open_document_file, translate_components};

mod v3_0;

mod yaml_wrapper;

#[derive(Debug)]
pub enum OpenApiDocument {
    V3_0(openapi_types::v3_0::Document),
}
