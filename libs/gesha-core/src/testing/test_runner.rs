use crate::gateway::testing::new_writer;
use crate::gateway::{detect_diff, file_to_string};
use crate::testing::{CanConvert, ConversionSetting, TestCase};
use crate::{Error, Result};
use futures::future::join_all;
use openapi_types::yaml::{load_from_str, ToOpenApi, YamlMap};
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
    pub async fn run(&self, cases: Vec<TestCase>) -> Result<()> {
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
}

#[instrument]
async fn run_single(case: TestCase) -> Result<()> {
    match case {
        TestCase::V3_0_Rust(rule) => {
            let writer = new_writer(&rule.output);
            let yaml = open_yaml_map(&rule.schema)?;
            let target = convert(yaml, &rule)?;
            writer.create_file(target)?;
            detect_diff(&rule.output, &rule.example)?;
            info!("passed: {path}", path = rule.schema.to_string_lossy());
        }
    }
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
