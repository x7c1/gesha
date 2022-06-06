use gesha_core::gateway;
use gesha_core::gateway::{test_rust_type, TestCase};
use gesha_core::targets::rust_type::Modules;
use openapi_types::v3_0;

#[derive(clap::Args, Debug)]
pub struct Params {
    #[clap(long)]
    schema: Option<String>,
}

pub fn run(params: Params) -> gateway::Result<()> {
    let cases: Vec<TestCase<(v3_0::ComponentsObject, Modules)>> = match params.schema {
        None => TestCase::from(vec![
            "struct.yaml",
            "numeric-fields.yaml",
            "boolean-field.yaml",
            "array.yaml",
            "ref-property.yaml",
            "ref-items.yaml",
            "optional-field.yaml",
            "newtype.yaml",
            "newtype-numeric.yaml",
            "reserved-keywords.yaml",
            "enum.yaml",
        ]),
        Some(x) => vec![TestCase::from_path(x)?],
    };
    cases.into_iter().try_for_each(test_rust_type)
}
