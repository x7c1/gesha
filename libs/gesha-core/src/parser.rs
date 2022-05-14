use openapi_types::v3_0::InfoObject;
use openapi_types::{v3_0, OpenApiDocument};
use std::path::PathBuf;

pub fn open_document_file<A: Into<PathBuf>>(path: A) -> OpenApiDocument {
    println!("open_document_file> open_document_file {:?}", path.into());
    OpenApiDocument::V3_0(v3_0::Document {
        openapi: "3.0.0".to_string(),
        info: InfoObject {
            title: "sample title".to_string(),
        },
    })
}
