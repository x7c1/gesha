use gesha_core::gateway;
use gesha_core::gateway::testing::{
    generate_module_file, generate_rust_type, test_rust_type, TestCase,
};
use gesha_core::gateway::{detect_diff, Error, ErrorTheme};
use gesha_core::targets::rust_type::Modules;
use openapi_types::v3_0;

#[derive(clap::Args, Debug)]
pub struct Params {
    #[clap(long)]
    schema: Option<String>,
}

type SupportedTestCase = TestCase<(v3_0::ComponentsObject, Modules)>;

pub fn run(params: Params) -> gateway::Result<()> {
    if let Some(schema) = params.schema {
        let case = SupportedTestCase::from_path(schema)?;
        test_rust_type(case)?;
        return Ok(());
    }
    let cases = new_test_cases();
    cases.clone().into_iter().try_for_each(test_rust_type)?;
    generate_module_file("examples/v3.0/components.rs", cases)
}

fn new_test_cases() -> Vec<SupportedTestCase> {
    SupportedTestCase::from(vec![
        "struct_simple.yaml",
        "numeric_fields.yaml",
        "boolean_field.yaml",
        "array.yaml",
        "ref_property.yaml",
        "ref_items.yaml",
        "optional_field.yaml",
        "newtype.yaml",
        "newtype_numeric.yaml",
        "reserved_keywords.yaml",
        "enums.yaml",
        "all_of.yaml",
        "all_of_ref.yaml",
    ])
}

pub fn overwrite() -> gateway::Result<()> {
    let cases = new_test_cases()
        .into_iter()
        .filter_map(|x| run_and_catch_diff(x).transpose())
        .collect::<gateway::Result<Vec<ModifiedCase>>>()?;

    for case in cases {
        println!("Diff detected: {} {}", case.case.module_name, case.diff);
    }

    Ok(())
}

fn run_and_catch_diff(case: SupportedTestCase) -> gateway::Result<Option<ModifiedCase>> {
    generate_rust_type(case.clone())?;
    match detect_diff(&case.example, &case.output) {
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
