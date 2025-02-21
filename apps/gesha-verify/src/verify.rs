use clap::Parser;
use gesha_core::conversions::{TestDefinition, TestRunner};
use gesha_core::Result;
use gesha_rust_shapes::v3_0;
use tracing::instrument;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    pub schema: Option<String>,

    #[arg(long)]
    pub overwrite: bool,
}

#[instrument(name = "verify::run")]
pub async fn run(args: Args) -> Result<()> {
    let converter = v3_0::Converter::default();
    process(converter, args).await
}

async fn process<A: TestDefinition>(definition: A, args: Args) -> Result<()> {
    let cases = if let Some(schema) = args.schema {
        vec![definition.require_test_case(&schema)?]
    } else {
        definition.list_test_cases()
    };
    let runner = TestRunner::new(definition);
    runner.run_tests(cases).await
}
