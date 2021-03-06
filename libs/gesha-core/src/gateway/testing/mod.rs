mod generate_module_file;
pub use generate_module_file::generate_module_file;

pub mod v3_0;

use crate::conversions::{ToOpenApi, ToRustType};
use crate::gateway;
use crate::gateway::{detect_diff, Reader, Writer};
use crate::renderer::Renderer;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TestCase<A> {
    pub output: PathBuf,
    pub schema: PathBuf,
    pub example: PathBuf,
    pub module_name: String,
    phantom: PhantomData<A>,
}

impl<A> Clone for TestCase<A> {
    fn clone(&self) -> Self {
        Self {
            output: self.output.clone(),
            schema: self.schema.clone(),
            example: self.example.clone(),
            module_name: self.module_name.clone(),
            phantom: Default::default(),
        }
    }
}

fn generate_rust_type<A, B>(target: TestCase<(A, B)>) -> gateway::Result<()>
where
    A: Debug + ToOpenApi,
    B: Debug + ToRustType<A> + Renderer,
{
    println!("target> {:#?}", target);

    let reader = Reader::new::<A>();
    let rust_types: B = reader.open_rust_type(target.schema)?;
    println!("rust_types> {:#?}", rust_types);

    let writer = new_writer(target.output);
    writer.create_file(rust_types)
}

pub fn test_rust_types<X, A, B>(targets: X) -> gateway::Result<()>
where
    X: Into<Vec<TestCase<(A, B)>>>,
    A: Debug + ToOpenApi,
    B: Debug + ToRustType<A> + Renderer,
{
    targets.into().into_iter().try_for_each(test_rust_type)
}

pub fn test_rust_type<X, A, B>(target: X) -> gateway::Result<()>
where
    X: Into<TestCase<(A, B)>>,
    A: Debug + ToOpenApi,
    B: Debug + ToRustType<A> + Renderer,
{
    let target = target.into();
    generate_rust_type(target.clone())?;
    detect_diff(&target.output, &target.example)
}

pub fn test_rust_type_to_overwrite<A, B>(target: TestCase<(A, B)>) -> gateway::Result<()>
where
    A: Debug + ToOpenApi,
    B: Debug + ToRustType<A> + Renderer,
{
    generate_rust_type(target.clone())?;

    // example doesn't exist at first attempt.
    let not_exist = !target.example.exists();
    if not_exist {
        new_writer(&target.example).touch()?;
    }

    // contrary to test_rust_type(),
    // target.example is actual file, target.output modified is expected file.
    detect_diff(&target.example, &target.output)
}

pub fn new_writer<A: Into<PathBuf>>(path: A) -> Writer {
    Writer {
        path: path.into(),
        preamble: Some("/*\n    Generated by gesha command; DO NOT EDIT BY HAND!\n*/".to_string()),
    }
}
