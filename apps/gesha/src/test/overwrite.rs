use crate::test;
use crate::test::SupportedTestCase;
use gesha_core::gateway;
use gesha_core::gateway::testing::test_rust_type_to_overwrite;
use gesha_core::gateway::{Error, ErrorTheme};

pub fn overwrite() -> gateway::Result<()> {
    let cases = test::new_test_cases()
        .into_iter()
        .filter_map(|x| run_and_catch_diff(x).transpose())
        .collect::<gateway::Result<Vec<ModifiedCase>>>()?;

    for case in cases {
        println!("Diff detected: {} {}", case.case.module_name, case.diff);
    }

    Ok(())
}

fn run_and_catch_diff(case: SupportedTestCase) -> gateway::Result<Option<ModifiedCase>> {
    match test_rust_type_to_overwrite(case.clone()) {
        Ok(_) => Ok(None),
        Err(e @ Error::DiffDetected { .. }) => Ok(Some(ModifiedCase {
            case,
            diff: e.detail(ErrorTheme::Overwrite),
        })),
        Err(e) => Err(e),
    }
}

struct ModifiedCase {
    case: SupportedTestCase,
    diff: String,
}
