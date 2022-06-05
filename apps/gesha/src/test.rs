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
            "optional-field.yaml",
            "numeric-fields.yaml",
            "boolean-field.yaml",
            "array.yaml",
            "ref-property.yaml",
            "ref-items.yaml",
            "newtype.yaml",
            "newtype-numeric.yaml",
        ]),
        Some(x) => to_cases(x),
    };
    cases.into_iter().try_for_each(test_rust_type)
}

fn to_cases(path: String) -> Vec<TestCase<(v3_0::ComponentsObject, Modules)>> {
    let target = "examples/v3.0/components/";
    if path.starts_with(target) {
        return TestCase::from(vec![path.replace(target, "")]);
    }
    unimplemented!("unsupported location: {}", path)
}
