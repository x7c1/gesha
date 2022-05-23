use crate::conversions::ToOpenApi;
use crate::conversions::ToRustType;
use crate::yaml_wrapper::{load_map_from_str, YamlMap};
use std::fs::File;
use std::io::Read;
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
    pub fn open_rust_type<P, B>(&self, path: P) -> crate::Result<B>
    where
        P: Into<PathBuf>,
        B: ToRustType<A>,
    {
        let map = open_yaml_map(path)?;
        let openapi_value = ToOpenApi::apply(map)?;
        ToRustType::apply(openapi_value)
    }
}

// TODO: remove unwrap
fn open_yaml_map<A: Into<PathBuf>>(path: A) -> crate::Result<YamlMap> {
    let path = path.into();
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    load_map_from_str(&contents)
}
