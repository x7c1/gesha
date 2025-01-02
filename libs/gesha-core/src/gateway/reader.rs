use crate::Error::CannotReadFile;
use crate::{Error, Result};
use gesha_rust_shapes::ToRustType;
use gesha_rust_types::SourceCode;
use openapi_types::yaml::{load_from_str, ToOpenApi, YamlMap};
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
    pub fn open_rust_type<P>(&self, path: P) -> Result<SourceCode>
    where
        P: AsRef<Path> + Debug,
        A: ToRustType,
    {
        let path = path.as_ref();
        let map = open_yaml_map(path)?;
        let openapi_value: A = ToOpenApi::apply(map).map_err(Error::openapi(path))?;
        let code = ToRustType::apply(openapi_value).map_err(Error::shapes(path))?;
        Ok(code)
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
    let content = file_to_string(path.as_ref())?;
    let map = load_from_str(&content).map_err(Error::openapi(path.as_ref()))?;
    Ok(map)
}
