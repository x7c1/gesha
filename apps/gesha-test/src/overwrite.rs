use crate::test::Args;
use gesha_core::conversions::{TestDefinition, TestRunner};
use gesha_core::Result;
use gesha_rust_shapes::v3_0;
use tracing::{info, instrument};

#[instrument(name = "overwrite::run")]
pub async fn run(args: Args) -> Result<()> {
    let definition = v3_0::Definition::default();
    process(definition, args).await
}

async fn process<A: TestDefinition>(definition: A, args: Args) -> Result<()> {
    let cases = if let Some(schema) = args.schema {
        vec![definition.require_test_case(&schema)?]
    } else {
        definition.list_test_cases()
    };
    let test_suites = definition.test_suites();

    let runner = TestRunner::new(definition);
    let modified_cases = runner.collect_modified_cases(cases).await?;
    if modified_cases.is_empty() {
        info!("diff not detected");
    } else {
        runner.copy_modified_files(&modified_cases)?;
    }
    test_suites
        .iter()
        .try_for_each(|suite| runner.generate_test_suite_file(suite))
}
