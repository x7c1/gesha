mod generate_module_file;
pub use generate_module_file::generate_module_file;

pub mod v3_0;

use crate::conversions::{ToOpenApi, ToRustType};
use crate::gateway;
use crate::gateway::{detect_diff, Error, Reader, Writer};
use crate::renderer::Renderer;
use futures::future::join_all;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;
use tracing::{debug, info, instrument, Instrument};

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
    debug!("target> {:#?}", target);

    let reader = Reader::new::<A>();
    let rust_types: B = reader.open_rust_type(target.schema)?;
    debug!("rust_types> {:#?}", rust_types);

    let writer = new_writer(target.output);
    writer.create_file(rust_types)
}

#[instrument(skip_all)]
pub async fn test_rust_types<X, A, B>(targets: X) -> gateway::Result<()>
where
    X: Into<Vec<TestCase<(A, B)>>> + Debug,
    A: ToOpenApi + Debug + Send + 'static,
    B: ToRustType<A> + Debug + Renderer + Send + 'static,
{
    let run_tests = targets
        .into()
        .into_iter()
        .map(|x| tokio::spawn(test_rust_type(x).in_current_span()));

    let errors = join_all(run_tests)
        .await
        .into_iter()
        .flatten()
        .filter_map(|result| result.err())
        .collect::<Vec<Error>>();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::Errors(errors))
    }
}

#[instrument]
pub async fn test_rust_type<X, A, B>(target: X) -> gateway::Result<()>
where
    X: Into<TestCase<(A, B)>> + Debug,
    A: Debug + ToOpenApi,
    B: Debug + ToRustType<A> + Renderer,
{
    let target = target.into();
    generate_rust_type(target.clone())?;
    detect_diff(&target.output, &target.example)?;

    info!("done");
    Ok(())
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
