use crate::test::Args;
use gesha_core::conversion::{Definition, TestRunner};
use gesha_core::Result;
use gesha_rust_shapes::v3_0::RustTypes;
use tracing::{info, instrument};

#[instrument(name = "overwrite::run")]
pub async fn run(args: Args) -> Result<()> {
    let cases = if let Some(schema) = args.schema {
        vec![RustTypes::require_test_case(&schema)?]
    } else {
        RustTypes::list_test_cases()
    };
    let modified_cases = TestRunner::<RustTypes>::collect_modified_cases(cases).await?;
    if modified_cases.is_empty() {
        info!("diff not detected");
    } else {
        TestRunner::<RustTypes>::copy_modified_files(&modified_cases)?;
    }
    RustTypes::test_suites()
        .iter()
        .try_for_each(TestRunner::<RustTypes>::generate_test_suite_file)
}
