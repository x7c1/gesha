use crate::test;
use crate::test::Args;
use gesha_core::gateway;
use gesha_core::gateway::testing::v3_0::ComponentCase;
use gesha_core::gateway::testing::{generate_module_file, test_rust_types_to_overwrite};
use gesha_core::gateway::Writer;
use tracing::instrument;

#[instrument(name = "test::overwrite")]
pub async fn run(args: Args) -> gateway::Result<()> {
    let test_cases = if let Some(schema) = args.schema {
        let case = ComponentCase::from_path(schema)?;
        vec![case.into()]
    } else {
        test::new_schemas_cases().into()
    };
    let cases = test_rust_types_to_overwrite(test_cases).await?;
    for case in cases.iter() {
        println!("Diff detected: {} {}", case.target.module_name, case.diff);
    }
    for case in cases {
        let writer = Writer {
            path: case.target.example,
            preamble: None,
        };
        writer.copy_file(case.target.output)?;
    }
    test::all_cases().into_iter().try_for_each(|cases| {
        let module_path = cases.module_path.clone();
        generate_module_file(module_path, cases.into())
    })
}
