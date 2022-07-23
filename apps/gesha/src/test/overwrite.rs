use crate::test;
use crate::test::SupportedTestCase;
use gesha_core::gateway;
use gesha_core::gateway::testing::v3_0::ComponentCase;
use gesha_core::gateway::testing::{generate_module_file, test_rust_type_to_overwrite};
use gesha_core::gateway::{Error, ErrorTheme, Writer};

#[derive(clap::Args, Debug)]
pub struct Params {
    #[clap(long)]
    schema: Option<String>,
}

pub fn run(params: Params) -> gateway::Result<()> {
    let test_cases = if let Some(schema) = params.schema {
        let case = ComponentCase::from_path(schema)?;
        vec![case.into()]
    } else {
        test::new_schemas_cases().into()
    };

    let cases = test_cases
        .into_iter()
        .filter_map(|x| run_and_catch_diff(x).transpose())
        .collect::<gateway::Result<Vec<ModifiedCase>>>()?;

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

    let schema_cases = test::new_schemas_cases();
    generate_module_file(schema_cases.module_path(), schema_cases.into())
}

fn run_and_catch_diff(target: SupportedTestCase) -> gateway::Result<Option<ModifiedCase>> {
    match test_rust_type_to_overwrite(target.clone()) {
        Ok(_) => Ok(None),
        Err(e @ Error::DiffDetected { .. }) => Ok(Some(ModifiedCase {
            target,
            diff: e.detail(ErrorTheme::Overwrite),
        })),
        Err(e) => Err(e),
    }
}

struct ModifiedCase {
    target: SupportedTestCase,
    diff: String,
}
