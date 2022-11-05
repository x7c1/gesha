pub mod overwrite;

use gesha_core::gateway;
use gesha_core::gateway::testing::v3_0::ComponentKind::{RequestBodies, Schemas};
use gesha_core::gateway::testing::v3_0::{ComponentCase, ComponentCases};
use gesha_core::gateway::testing::{test_rust_type, test_rust_types, TestCase};
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
        let case = ComponentCase::from_path(schema)?;
        test_rust_type(case)?;
        return Ok(());
    }
    all_cases().into_iter().try_for_each(test_rust_types)
}

fn new_schemas_cases() -> ComponentCases {
    ComponentCases::from_vec(
        Schemas,
        vec![
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
            "camel_case_fields.yaml",
            "title_description.yaml",
            "nullable_field.yaml",
        ],
    )
}

fn new_request_bodies_cases() -> ComponentCases {
    ComponentCases::from_vec(RequestBodies, vec!["schema_ref.yaml"])
}

fn all_cases() -> Vec<ComponentCases> {
    vec![new_schemas_cases(), new_request_bodies_cases()]
}
