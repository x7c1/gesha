use crate::conversions::{Definition, TestCase, TestCaseMap, TestSuite};
use crate::io::{detect_diff, Reader, Writer};
use crate::{Error, ErrorTheme, Result};
use futures::future::join_all;
use openapi_types::yaml::ToOpenApi;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use tracing::Instrument;
use tracing::{info, instrument};

#[derive(Debug, Default)]
pub struct TestRunner<A>(PhantomData<A>);

impl<A> TestRunner<A>
where
    A: Definition + Send + Sync + 'static,
    A::OpenApiType: ToOpenApi + Send + Sync + 'static,
    A::TargetType: Display + Send + Sync + 'static,
{
    #[instrument(skip_all)]
    pub async fn run_tests(cases: Vec<TestCase<A::OpenApiType, A::TargetType>>) -> Result<()> {
        let (run_tests, mut map) = cases
            .into_iter()
            .map(|case| {
                let cloned_case = case.clone();
                let handle = tokio::spawn(Self::run_single(case).in_current_span());
                (handle.id(), cloned_case, handle)
            })
            .fold((vec![], TestCaseMap::new()), TestCaseMap::accumulate);

        let errors =
            join_all(run_tests)
                .await
                .into_iter()
                .try_fold(vec![], |mut errors, result| {
                    match result {
                        Ok(Ok(_)) => { /* nop */ }
                        Ok(Err(e)) => errors.push(e),
                        Err(cause) => errors.push(Error::JoinError {
                            schema_path: map.extract(cause.id())?.schema,
                            cause,
                        }),
                    }
                    Ok(errors)
                })?;

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Errors(errors))
        }
    }

    #[instrument(skip_all)]
    pub async fn collect_modified_cases(
        cases: Vec<TestCase<A::OpenApiType, A::TargetType>>,
    ) -> Result<Vec<ModifiedTestCase<A::OpenApiType, A::TargetType>>> {
        let (run_tests, mut map) = cases
            .into_iter()
            .map(|case| {
                let cloned_case = case.clone();
                let handle = tokio::spawn(Self::detect_modified_case(case).in_current_span());
                (handle.id(), cloned_case, handle)
            })
            .fold((vec![], TestCaseMap::new()), TestCaseMap::accumulate);

        let (modified, errors) = join_all(run_tests).await.into_iter().try_fold(
            (vec![], vec![]),
            |(mut modified, mut errors), result| {
                match result {
                    Ok(Ok(Some(case))) => modified.push(case),
                    Ok(Ok(None)) => { /* nop */ }
                    Ok(Err(e)) => errors.push(e),
                    Err(cause) => errors.push(Error::JoinError {
                        schema_path: map.extract(cause.id())?.schema,
                        cause,
                    }),
                }
                Ok((modified, errors))
            },
        )?;

        if errors.is_empty() {
            Ok(modified)
        } else {
            Err(Error::Errors(errors))
        }
    }

    #[instrument(skip_all)]
    pub fn copy_modified_files(
        cases: &[ModifiedTestCase<A::OpenApiType, A::TargetType>],
    ) -> Result<()> {
        cases
            .iter()
            .try_for_each(|case| Self::copy_modified_file(case))
    }

    pub fn generate_test_suite_file(
        suite: &TestSuite<A::OpenApiType, A::TargetType>,
    ) -> Result<()> {
        let writer = Writer::new(&suite.mod_path);
        let content = A::test_suites_content(suite);
        writer.create_file(content)
    }

    async fn run_single(case: TestCase<A::OpenApiType, A::TargetType>) -> Result<()> {
        let writer = Writer::new(&case.output);
        let reader = Reader::new(&case.schema);
        let target = reader.open_target_type::<A>()?;
        writer.create_file(target)?;

        detect_diff(&case.output, &case.example)?;
        info!("passed: {path}", path = case.schema.to_string_lossy());
        Ok(())
    }

    fn copy_modified_file(case: &ModifiedTestCase<A::OpenApiType, A::TargetType>) -> Result<()> {
        info!("diff detected: {} {}", case.target.module_name, case.diff);
        let writer = Writer::new(&case.target.example);
        writer.copy_from(&case.target.output)
    }

    async fn detect_modified_case(
        case: TestCase<A::OpenApiType, A::TargetType>,
    ) -> Result<Option<ModifiedTestCase<A::OpenApiType, A::TargetType>>> {
        let writer = Writer::new(&case.output);
        let reader = Reader::new(&case.schema);
        let target = reader.open_target_type::<A>()?;
        writer.create_file(target)?;

        // The example is not available on the first attempt.
        let not_exist = !case.example.exists();
        if not_exist {
            Writer::new(&case.example).touch()?;
        }

        // Unlike run_single(), case.example represents the actual file,
        // while case.output modified represents the expected file.
        let result = detect_diff(&case.example, &case.output);
        match result {
            Ok(_) => Ok(None),
            Err(e @ Error::DiffDetected { .. }) => Ok(Some(ModifiedTestCase {
                target: case,
                diff: e.detail(ErrorTheme::Overwrite),
            })),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub struct ModifiedTestCase<From, To> {
    pub target: TestCase<From, To>,
    pub diff: String,
}
