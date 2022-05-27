use crate::conversions::ToOpenApi;
use crate::conversions::ToRustType;
use crate::io::Error::CannotReadFile;
use crate::io::Result;
use crate::yaml_wrapper::{load_map_from_str, YamlMap};
use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;

pub struct Reader<A>(PhantomData<A>);

impl Reader<()> {
    pub fn new<A>() -> Reader<A> {
        Reader(PhantomData::default())
    }
}

impl<A> Reader<A>
where
    A: ToOpenApi,
{
    pub fn open_rust_type<P, B>(&self, path: P) -> Result<B>
    where
        P: Into<PathBuf>,
        B: ToRustType<A>,
    {
        let map = open_yaml_map(path)?;
        let openapi_value = ToOpenApi::apply(map)?;
        let rust_type = ToRustType::apply(openapi_value)?;
        Ok(rust_type)
    }
}

fn open_yaml_map<A: Into<PathBuf>>(path: A) -> Result<YamlMap> {
    let path = path.into();
    let content = fs::read_to_string(&path).map_err(|cause| CannotReadFile {
        path: path.clone(),
        detail: format!("{:?}", cause),
    })?;
    let map = load_map_from_str(&content)?;
    Ok(map)
}
