use crate::test::Args;
use gesha_core::testing::{TestCase, TestCasesParent, TestRunner};
use gesha_core::Result;
use tracing::{info, instrument};

#[instrument(name = "overwrite::run")]
pub async fn run(args: Args) -> Result<()> {
    let cases = if let Some(schema) = args.schema {
        vec![TestCase::require(&schema)?]
    } else {
        TestCase::all()
    };
    let runner = TestRunner::new();
    let modified_cases = runner.collect_modified_cases(cases).await?;
    if modified_cases.is_empty() {
        info!("diff not detected");
    } else {
        runner.copy_modified_files(&modified_cases)?;
    }
    TestCasesParent::all()
        .iter()
        .try_for_each(|parent| runner.generate_mod_file(parent))
}
