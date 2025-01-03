use crate::testing::v3_0::COMPONENTS_PATH;
use crate::testing::{CanConvert, ConversionError, ConversionSetting, TestCase, TestCasesParent};
use gesha_rust_shapes::ToRustType;
use openapi_types::v3_0;

impl CanConvert<v3_0::ComponentsObject> for gesha_rust_types::SourceCode {
    fn convert(x: v3_0::ComponentsObject) -> Result<Self, ConversionError> {
        ToRustType::apply(x).map_err(ConversionError::RustShape)
    }

    fn test_case_parents() -> Vec<TestCasesParent> {
        // TODO: add request_bodies
        vec![schemas()]
    }
}

impl ConversionSetting<v3_0::ComponentsObject, gesha_rust_types::SourceCode> {
    pub fn v3_0_rust(yaml_name: &str) -> Self {
        let rs_name = yaml_name.replace(".yaml", ".rs");
        // TODO: support request_bodies
        let dir = "schemas";
        Self {
            output: format!("output/v3.0/components/{dir}/{rs_name}").into(),
            schema: format!("{COMPONENTS_PATH}/{dir}/{yaml_name}").into(),
            example: format!("{COMPONENTS_PATH}/{dir}/{rs_name}").into(),
            module_name: yaml_name.replace(".yaml", ""),
            phantom: Default::default(),
        }
    }
}

fn schemas() -> TestCasesParent {
    let filenames = vec![
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
    ];
    let enclosed_cases = filenames
        .iter()
        .map(|filename| TestCase::V3_0_Rust(ConversionSetting::v3_0_rust(filename)))
        .collect();

    let path = format!(
        "examples/v3_0/src/components/{module}.rs",
        module = "schemas"
    );
    TestCasesParent {
        file_path: path.into(),
        enclosed_cases,
    }
}

/*
fn new_request_bodies_cases() -> ComponentCases {
    ComponentCases::from_vec(RequestBodies, vec!["schema_ref.yaml"])
}
*/
