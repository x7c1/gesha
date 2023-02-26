use crate::test;
use crate::test::Args;
use gesha_core::gateway;
use gesha_core::gateway::testing::v3_0::ComponentCase;
use gesha_core::gateway::testing::{collect_modified_cases, generate_module_file};
use gesha_core::gateway::Writer;
use tracing::{info, instrument};

#[instrument(name = "overwrite::run")]
pub async fn run(args: Args) -> gateway::Result<()> {
    let test_cases = if let Some(schema) = args.schema {
        let case = ComponentCase::from_path(schema)?;
        vec![case.into()]
    } else {
        test::new_schemas_cases().into()
    };
    let cases = collect_modified_cases(test_cases).await?;
    if cases.is_empty() {
        info!("diff not detected");
    } else {
        for case in cases.iter() {
            info!("diff detected: {} {}", case.target.module_name, case.diff);
        }
        for case in cases {
            let writer = Writer {
                path: case.target.example,
                preamble: None,
            };
            writer.copy_file(case.target.output)?;
        }
    }
    test::all_cases().into_iter().try_for_each(|cases| {
        let module_path = cases.module_path.clone();
        generate_module_file(module_path, cases.into())
    })
}
