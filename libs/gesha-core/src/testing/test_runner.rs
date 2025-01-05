use crate::gateway::{detect_diff, file_to_string, Writer};
use crate::testing::{CanConvert, ConversionSetting, TestCase, TestCasesParent};
use crate::{Error, ErrorTheme, Result};
use futures::future::join_all;
use gesha_rust_types::{ModuleDeclarations, ModuleName};
use openapi_types::yaml::{load_from_str, ToOpenApi, YamlMap};
use std::fmt::{Debug, Display};
use std::path::Path;
use tracing::Instrument;
use tracing::{info, instrument};

#[derive(Debug, Default)]
pub struct TestRunner {}

impl TestRunner {
    pub fn new() -> Self {
        Self {}
    }
}

impl TestRunner {
    #[instrument(skip_all)]
    pub async fn run_tests(&self, cases: Vec<TestCase>) -> Result<()> {
        let run_tests = cases
            .into_iter()
            .map(|case| tokio::spawn(run_single(case).in_current_span()));

        let errors = join_all(run_tests)
            .await
            .into_iter()
            .flatten()
            .filter_map(|result| result.err())
            .collect::<Vec<_>>();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Errors(errors))
        }
    }

    #[instrument(skip_all)]
    pub async fn collect_modified_cases(
        &self,
        cases: Vec<TestCase>,
    ) -> Result<Vec<ModifiedTestCase>> {
        let run_tests = cases
            .into_iter()
            .map(|x| tokio::spawn(detect_modified_case(x).in_current_span()));

        let init = (vec![], vec![]);
        let (modified, errors) = join_all(run_tests).await.into_iter().fold(
            init,
            |(mut modified, mut errors), result| {
                match result {
                    Ok(Ok(Some(x))) => modified.push(x),
                    Ok(Ok(None)) => { /* nop */ }
                    Ok(Err(e)) => errors.push(e),
                    Err(e) => errors.push(Error::JoinError(e)),
                }
                (modified, errors)
            },
        );
        if errors.is_empty() {
            Ok(modified)
        } else {
            Err(Error::Errors(errors))
        }
    }

    #[instrument(skip_all)]
    pub fn copy_modified_files(&self, cases: &[ModifiedTestCase]) -> Result<()> {
        cases
            .iter()
            .try_for_each(|case| self.copy_modified_file(case))
    }

    #[instrument(skip_all)]
    pub fn copy_modified_file(&self, case: &ModifiedTestCase) -> Result<()> {
        match &case.target {
            TestCase::V3_0_Rust(setting) => {
                info!("diff detected: {} {}", setting.module_name, case.diff);
                let writer = Writer {
                    path: setting.example.clone(),
                    preamble: None,
                };
                writer.copy_file(&setting.output)?;
            }
        }
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn generate_mod_file(&self, parent: &TestCasesParent) -> Result<()> {
        let writer = Writer::new(&parent.file_path);
        let decls = parent
            .enclosed_cases
            .iter()
            .map(|case| case.module_name())
            .map(ModuleName::new)
            .collect::<ModuleDeclarations>();

        writer.create_file(decls)
    }
}

#[instrument]
async fn run_single(case: TestCase) -> Result<()> {
    match case {
        TestCase::V3_0_Rust(rule) => {
            write_file(&rule)?;
            detect_diff(&rule.output, &rule.example)?;
            info!("passed: {path}", path = rule.schema.to_string_lossy());
        }
    }
    Ok(())
}

#[instrument]
async fn detect_modified_case(case: TestCase) -> Result<Option<ModifiedTestCase>> {
    let result = match &case {
        TestCase::V3_0_Rust(rule) => {
            write_file(rule)?;

            // example doesn't exist at first attempt.
            let not_exist = !rule.example.exists();
            if not_exist {
                Writer::new(&rule.example).touch()?;
            }

            // contrary to run_single(),
            // rule.example is actual file, rule.output modified is expected file.
            detect_diff(&rule.example, &rule.output)
        }
    };
    match result {
        Ok(_) => Ok(None),
        Err(e @ Error::DiffDetected { .. }) => Ok(Some(ModifiedTestCase {
            target: case,
            diff: e.detail(ErrorTheme::Overwrite),
        })),
        Err(e) => Err(e),
    }
}

fn write_file<A, B>(rule: &ConversionSetting<A, B>) -> Result<()>
where
    A: ToOpenApi,
    B: CanConvert<A> + Display + Debug,
{
    let writer = Writer::new(&rule.output);
    let yaml = open_yaml_map(&rule.schema)?;
    let target = convert(yaml, rule)?;
    writer.create_file(target)?;
    Ok(())
}

fn convert<From, To>(yaml: YamlMap, rule: &ConversionSetting<From, To>) -> Result<To>
where
    To: CanConvert<From>,
    From: ToOpenApi,
{
    let x: From = ToOpenApi::apply(yaml).map_err(Error::openapi(&rule.schema))?;
    let y: To = CanConvert::convert(x).map_err(Error::conversion(&rule.schema))?;
    Ok(y)
}

fn open_yaml_map<A: AsRef<Path>>(path: A) -> Result<YamlMap> {
    let content = file_to_string(path.as_ref())?;
    let map = load_from_str(&content).map_err(Error::openapi(path.as_ref()))?;
    Ok(map)
}

#[derive(Debug)]
pub struct ModifiedTestCase {
    pub target: TestCase,
    pub diff: String,
}
