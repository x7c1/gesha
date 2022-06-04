use gesha_core::gateway;
use gesha_core::gateway::{test_rust_type, TestCase};
use gesha_core::targets::rust_type::Modules;
use openapi_types::v3_0;

pub fn run() -> gateway::Result<()> {
    let cases = TestCase::<(v3_0::ComponentsObject, Modules)>::from(vec![
        "pet.yaml",
        "optional-field.yaml",
        "numeric-fields.yaml",
        "boolean-field.yaml",
        "array.yaml",
        "ref-property.yaml",
    ]);
    cases.into_iter().try_for_each(test_rust_type)
}
