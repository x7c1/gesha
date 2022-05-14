use crate::{v3_0, OpenApiDocument};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use yaml_rust::YamlLoader;

// TODO: remove unwrap
pub fn open_document_file<A: Into<PathBuf>>(path: A) -> crate::Result<OpenApiDocument> {
    let path = path.into();
    println!("open_document_file> open_document_file {:?}", path);

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut yamls = YamlLoader::load_from_str(&contents).unwrap();
    let yaml = yamls.swap_remove(0);

    v3_0::to_document(yaml)
}
