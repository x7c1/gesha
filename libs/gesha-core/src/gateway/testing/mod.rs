mod generate_module_file;
pub use generate_module_file::generate_module_file;

pub mod v3_0;

use crate::gateway::{detect_diff, Reader, Writer};
use crate::{Error, ErrorTheme, Result};
use futures::future::join_all;
use gesha_rust_shapes::ToRustType;
use openapi_types::yaml::ToOpenApi;
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

fn generate_rust_type<A>(target: TestCase<A>) -> Result<()>
where
    A: Debug + ToOpenApi,
    A: Debug + ToRustType,
{
    debug!("target> {:#?}", target);

    let reader = Reader::new::<A>();
    let code = reader.open_rust_type(target.schema)?;
    debug!("rust_types> {:#?}", code);

    let writer = new_writer(target.output);
    writer.create_file(code)
}

#[instrument(skip_all)]
pub async fn test_rust_types<X, A>(targets: X) -> Result<()>
where
    X: Into<Vec<TestCase<A>>> + Debug,
    A: ToOpenApi + Debug + Send + 'static,
    A: ToRustType + Debug + Send + 'static,
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
pub async fn test_rust_type<X, A>(target: X) -> Result<()>
where
    X: Into<TestCase<A>> + Debug,
    A: Debug + ToOpenApi,
    A: Debug + ToRustType,
{
    let target = target.into();
    generate_rust_type(target.clone())?;
    detect_diff(&target.output, &target.example)?;

    info!("passed: {path}", path = target.schema.to_string_lossy());
    Ok(())
}

#[instrument(skip_all)]
pub async fn collect_modified_cases<A>(cases: Vec<TestCase<A>>) -> Result<Vec<ModifiedTestCase<A>>>
where
    A: Debug + ToOpenApi + Send + 'static,
    A: Debug + ToRustType + Send + 'static,
{
    let run_tests = cases
        .into_iter()
        .map(|x| tokio::spawn(detect_modified_case(x).in_current_span()));

    let init = (vec![], vec![]);
    let (modified, errors) =
        join_all(run_tests)
            .await
            .into_iter()
            .fold(init, |(mut modified, mut errors), result| {
                match result {
                    Ok(Ok(Some(x))) => modified.push(x),
                    Ok(Ok(None)) => { /* nop */ }
                    Ok(Err(e)) => errors.push(e),
                    Err(e) => errors.push(Error::JoinError(e)),
                }
                (modified, errors)
            });

    if errors.is_empty() {
        Ok(modified)
    } else {
        Err(Error::Errors(errors))
    }
}

#[instrument]
pub async fn detect_modified_case<A>(case: TestCase<A>) -> Result<Option<ModifiedTestCase<A>>>
where
    A: Debug + ToOpenApi,
    A: Debug + ToRustType,
{
    let run = |target: TestCase<A>| {
        generate_rust_type(target.clone())?;

        // example doesn't exist at first attempt.
        let not_exist = !target.example.exists();
        if not_exist {
            new_writer(&target.example).touch()?;
        }

        // contrary to test_rust_type(),
        // target.example is actual file, target.output modified is expected file.
        detect_diff(&target.example, &target.output)
    };
    match run(case.clone()) {
        Ok(_) => Ok(None),
        Err(e @ Error::DiffDetected { .. }) => Ok(Some(ModifiedTestCase {
            target: case.clone(),
            diff: e.detail(ErrorTheme::Overwrite),
        })),
        Err(e) => Err(e),
    }
}

pub fn new_writer<A: Into<PathBuf>>(path: A) -> Writer {
    Writer {
        path: path.into(),
        preamble: Some("/*\n    Generated by gesha command; DO NOT EDIT BY HAND!\n*/".to_string()),
    }
}

#[derive(Debug)]
pub struct ModifiedTestCase<A> {
    pub target: TestCase<A>,
    pub diff: String,
}
