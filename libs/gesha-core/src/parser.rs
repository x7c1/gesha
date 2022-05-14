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
    v3_0::to_document(map)
}
