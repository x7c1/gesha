use crate::verify::Args;
use gesha_core::Result;
use gesha_core::testing::{TestDefinition, TestRunner};
use gesha_rust_shapes::v3_0;
use tracing::{info, instrument};

#[instrument(name = "overwrite::run")]
pub async fn run(args: Args) -> Result<()> {
    let converter = v3_0::testing::ComponentsConverter::default();
    process(converter, args).await
}

async fn process<A: TestDefinition>(definition: A, args: Args) -> Result<()> {
    let cases = if let Some(schema) = args.schema {
        vec![definition.require_test_case(&schema)?]
    } else {
        definition.list_test_cases()
    };
    let indexes = definition.list_indexes();
    let runner = TestRunner::new(definition);
    let modified_cases = runner.collect_modified_cases(cases).await?;
    if modified_cases.is_empty() {
        info!("diff not detected");
    } else {
        runner.copy_modified_files(&modified_cases)?;
    }
    runner.generate_test_index_files(&indexes)
}
