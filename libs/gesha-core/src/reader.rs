use crate::targets::rust::RustModules;
use crate::yaml_wrapper::load_map_from_str;
use crate::{v3_0, OpenApiDocument};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// TODO: remove unwrap
pub fn open_document_file<A: Into<PathBuf>>(path: A) -> crate::Result<OpenApiDocument> {
    let path = path.into();
    println!("open_document_file> open_document_file {:?}", path);

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let map = load_map_from_str(&contents)?;
    v3_0::openapi::to_document(map)
}

pub fn to_rust_modules(document: OpenApiDocument) -> crate::Result<Option<RustModules>> {
    let maybe = match document {
        OpenApiDocument::V3_0(doc) => doc.components.map(v3_0::to_rust::from_components),
    };
    maybe.transpose()
}
