use crate::targets::rust::RustModules;
use crate::v3_0::openapi::to_schemas;
use crate::yaml_wrapper::{load_map_from_str, YamlMap};
use crate::{v3_0, OpenApiDocument};
use openapi_types::v3_0::SchemasObject;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn open_document_file<A: Into<PathBuf>>(path: A) -> crate::Result<OpenApiDocument> {
    let map = open_yaml_map(path)?;
    v3_0::openapi::to_document(map)
}

pub fn open_v3_0_schemas_file<A: Into<PathBuf>>(path: A) -> crate::Result<SchemasObject> {
    let map = open_yaml_map(path)?;
    to_schemas(map)
}

// TODO: remove unwrap
fn open_yaml_map<A: Into<PathBuf>>(path: A) -> crate::Result<YamlMap> {
    let path = path.into();
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    load_map_from_str(&contents)
}

pub fn to_rust_modules(document: OpenApiDocument) -> crate::Result<Option<RustModules>> {
    let maybe = match document {
        OpenApiDocument::V3_0(doc) => doc.components.map(v3_0::to_rust::from_components),
    };
    maybe.transpose()
}
