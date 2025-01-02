use clap::Parser;
use gesha_core::gateway::testing::v3_0::ComponentCases;
use gesha_core::gateway::testing::v3_0::ComponentKind::{RequestBodies, Schemas};
use gesha_core::testing::{TestCase, TestRunner};
use gesha_core::Result;
use std::vec;
use tracing::instrument;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    pub schema: Option<String>,

    #[arg(long)]
    pub overwrite: bool,
}

// type SupportedTestCase = TestCase<v3_0::ComponentsObject>;

// #[instrument(name = "test::run")]
// pub async fn run(args: Args) -> Result<()> {
//     if let Some(schema) = args.schema {
//         let case = ComponentCase::from_path(schema)?;
//         test_rust_type(case).await?;
//         return Ok(());
//     }
//     let cases = all_cases()
//         .into_iter()
//         .flat_map(Vec::<SupportedTestCase>::from)
//         .collect::<Vec<_>>();
//
//     test_rust_types(cases).await
// }

#[instrument(name = "test::run")]
pub async fn run(args: Args) -> Result<()> {
    let cases = if let Some(schema) = args.schema {
        vec![TestCase::require(&schema)?]
    } else {
        TestCase::all()
    };
    let runner = TestRunner::new();
    runner.run(cases).await
}

fn new_schemas_cases() -> ComponentCases {
    ComponentCases::from_vec(
        Schemas,
        vec![
            "object_simple.yaml",
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
            "object_inline.yaml",
            "object_inline_nested.yaml",
            "object_inline_ref.yaml",
            "object_inline_all_of.yaml",
            "object_inline_nullable.yaml",
            "all_of_inline_all_of.yaml",
            "all_of_override_optional.yaml",
            "object_inline_enum.yaml",
            "one_of.yaml",
            "object_inline_one_of.yaml",
        ],
    )
}

fn new_request_bodies_cases() -> ComponentCases {
    ComponentCases::from_vec(RequestBodies, vec!["schema_ref.yaml"])
}

pub fn all_cases() -> Vec<ComponentCases> {
    vec![new_schemas_cases(), new_request_bodies_cases()]
}
