use clap::Parser;
use gesha_core::testing::{TestCase, TestRunner};
use gesha_core::Result;
use std::vec;
use tracing::instrument;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    pub schema: Option<String>,

    #[arg(long)]
    pub overwrite: bool,
}

#[instrument(name = "test::run")]
pub async fn run(args: Args) -> Result<()> {
    let cases = if let Some(schema) = args.schema {
        vec![TestCase::require(&schema)?]
    } else {
        TestCase::all()
    };
    let runner = TestRunner::new();
    runner.run_tests(cases).await
}
