use crate::conversions::ToOpenApi;
use crate::conversions::ToRustType;
use crate::gateway::Error::CannotReadFile;
use crate::gateway::{Error, Result};
use crate::yaml::{load_from_str, YamlMap};
use std::fmt::Debug;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;
use tracing::instrument;

#[derive(Debug)]
pub struct Reader<A>(PhantomData<A>);

impl Reader<()> {
    pub fn new<A>() -> Reader<A> {
        Reader(PhantomData)
    }
}

impl<A> Reader<A>
where
    A: ToOpenApi + Debug,
{
    #[instrument]
    pub fn open_rust_type<P, B>(&self, path: P) -> Result<B>
    where
        P: AsRef<Path> + Debug,
        B: ToRustType<A>,
    {
        let path = path.as_ref();
        let map = open_yaml_map(path)?;
        let openapi_value = ToOpenApi::apply(map).map_err(Error::conversion(path))?;
        let rust_type = ToRustType::apply(openapi_value).map_err(Error::conversion(path))?;
        Ok(rust_type)
    }
}

pub fn file_to_string<A: AsRef<Path>>(path: A) -> Result<String> {
    let content = fs::read_to_string(&path).map_err(|cause| CannotReadFile {
        path: path.as_ref().into(),
        detail: format!("{:?}", cause),
    })?;
    Ok(content)
}

fn open_yaml_map<A: AsRef<Path>>(path: A) -> Result<YamlMap> {
    let content = file_to_string(path)?;
    let map = load_from_str(&content)?;
    Ok(map)
}
