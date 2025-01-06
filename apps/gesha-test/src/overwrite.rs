use crate::test::Args;
use gesha_core::conversions::{Definition, TestRunner};
use gesha_core::Result;
use gesha_rust_shapes::v3_0;
use tracing::{info, instrument};

#[instrument(name = "overwrite::run")]
pub async fn run(args: Args) -> Result<()> {
    process::<v3_0::RustTypes>(args).await
}

async fn process<A: Definition>(args: Args) -> Result<()> {
    let cases = if let Some(schema) = args.schema {
        vec![A::require_test_case(&schema)?]
    } else {
        A::list_test_cases()
    };
    let modified_cases = TestRunner::<A>::collect_modified_cases(cases).await?;
    if modified_cases.is_empty() {
        info!("diff not detected");
    } else {
        TestRunner::<A>::copy_modified_files(&modified_cases)?;
    }
    A::test_suites()
        .iter()
        .try_for_each(TestRunner::<A>::generate_test_suite_file)
}
