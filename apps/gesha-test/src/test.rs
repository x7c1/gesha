use clap::Parser;
use gesha_core::conversions::{Definition, TestRunner};
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

#[instrument(name = "test::run")]
pub async fn run(args: Args) -> Result<()> {
    process::<v3_0::RustTypes>(args).await
}

async fn process<A: Definition>(args: Args) -> Result<()> {
    let cases = if let Some(schema) = args.schema {
        vec![A::require_test_case(&schema)?]
    } else {
        A::list_test_cases()
    };
    TestRunner::<A>::run_tests(cases).await
}
