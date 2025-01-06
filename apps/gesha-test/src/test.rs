use clap::Parser;
use gesha_core::conversions::{Definition, TestRunner};
use gesha_core::Result;
use gesha_rust_shapes::v3_0::RustTypes;
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
        vec![RustTypes::require_test_case(&schema)?]
    } else {
        RustTypes::list_test_cases()
    };
    TestRunner::<RustTypes>::run_tests(cases).await
}
