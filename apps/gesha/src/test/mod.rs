pub mod overwrite;

use gesha_core::gateway;
use gesha_core::gateway::testing::{test_rust_type, TestCase};
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
    cases.into_iter().try_for_each(test_rust_type)
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
