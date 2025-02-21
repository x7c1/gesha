use crate::io::{detect_diff, Reader, Writer};
use crate::testing::{TestCase, TestCaseMap, TestDefinition, TestSuite};
use crate::{Error, ErrorTheme, Result};
use futures::future::join_all;
use std::fmt::Debug;
use tracing::Instrument;
use tracing::{info, instrument};

#[derive(Clone, Debug)]
pub struct TestRunner<A>(A);

impl<A> TestRunner<A>
where
    A: TestDefinition,
{
    pub fn new(definition: A) -> Self {
        Self(definition)
    }

    #[instrument(skip_all)]
    pub async fn run_tests(&self, cases: Vec<TestCase<A>>) -> Result<()> {
        let (run, mut map) = cases
            .into_iter()
            .map(|case| {
                let cloned_case = case.clone();
                let handle = tokio::spawn(self.clone().run_single_test(case).in_current_span());
                (handle.id(), cloned_case, handle)
            })
            .fold((vec![], TestCaseMap::new()), TestCaseMap::accumulate);

        let errors = join_all(run)
            .await
            .into_iter()
            .map(|result| map.flatten(result))
            .fold(vec![], |mut errors, result| {
                match result {
                    Ok(_) => { /* nop */ }
                    Err(e) => errors.push(e),
                }
                errors
            });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Errors(errors))
        }
    }

    #[instrument(skip_all)]
    pub async fn collect_modified_cases(
        &self,
        cases: Vec<TestCase<A>>,
    ) -> Result<Vec<ModifiedTestCase<A>>> {
        let (run, mut map) = cases
            .into_iter()
            .map(|case| {
                let cloned_case = case.clone();
                let handle =
                    tokio::spawn(self.clone().detect_modified_case(case).in_current_span());
                (handle.id(), cloned_case, handle)
            })
            .fold((vec![], TestCaseMap::new()), TestCaseMap::accumulate);

        let (modified, errors) = join_all(run)
            .await
            .into_iter()
            .map(|result| map.flatten(result))
            .fold((vec![], vec![]), |(mut modified, mut errors), result| {
                match result {
                    Ok(Some(case)) => modified.push(case),
                    Ok(None) => { /* nop */ }
                    Err(e) => errors.push(e),
                }
                (modified, errors)
            });

        if errors.is_empty() {
            Ok(modified)
        } else {
            Err(Error::Errors(errors))
        }
    }

    #[instrument(skip_all)]
    pub fn copy_modified_files(&self, cases: &[ModifiedTestCase<A>]) -> Result<()> {
        cases
            .iter()
            .try_for_each(|case| self.copy_modified_file(case))
    }

    #[instrument]
    pub fn generate_test_suite_file(&self, suite: &TestSuite<A>) -> Result<()> {
        let writer = Writer::new(&self.0, &suite.mod_path);
        let content = self.0.test_suite_code(suite);
        writer.write_code(content)
    }

    #[instrument(skip_all)]
    pub fn generate_test_suite_files(&self, suites: &[TestSuite<A>]) -> Result<()> {
        suites
            .iter()
            .try_for_each(|suite| self.generate_test_suite_file(suite))
    }

    #[instrument]
    async fn run_single_test(self, case: TestCase<A>) -> Result<()> {
        let writer = Writer::new(&self.0, &case.output);
        let reader = Reader::new(&case.schema);
        let target = reader.open_target_type(&self.0)?;
        writer.write_code(target)?;

        detect_diff(&case.output, &case.example)?;
        info!("passed: {path}", path = case.schema.to_string_lossy());
        Ok(())
    }

    #[instrument(skip_all)]
    fn copy_modified_file(&self, case: &ModifiedTestCase<A>) -> Result<()> {
        info!("diff detected: {} {}", case.target.module_name, case.diff);
        let writer = Writer::new(&self.0, &case.target.example);
        writer.copy_from(&case.target.output)
    }

    #[instrument]
    async fn detect_modified_case(self, case: TestCase<A>) -> Result<Option<ModifiedTestCase<A>>> {
        let writer = Writer::new(&self.0, &case.output);
        let reader = Reader::new(&case.schema);
        let target = reader.open_target_type(&self.0)?;
        writer.write_code(target)?;

        // The example is not available on the first attempt.
        let not_exist = !case.example.exists();
        if not_exist {
            Writer::new(&self.0, &case.example).touch()?;
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
pub struct ModifiedTestCase<A> {
    pub target: TestCase<A>,
    pub diff: String,
}
