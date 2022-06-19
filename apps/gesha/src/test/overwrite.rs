use crate::test;
use crate::test::SupportedTestCase;
use gesha_core::gateway;
use gesha_core::gateway::testing::test_rust_type_to_overwrite;
use gesha_core::gateway::{Error, ErrorTheme, Writer};

pub fn overwrite() -> gateway::Result<()> {
    let cases = test::new_test_cases()
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
    Ok(())
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
