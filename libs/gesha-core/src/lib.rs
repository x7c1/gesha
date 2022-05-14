mod error;
pub use error::{Error, Result};

mod parser;
pub use parser::open_document_file;

mod v3_0;

#[derive(Debug)]
pub enum OpenApiDocument {
    V3_0(openapi_types::v3_0::Document),
}
