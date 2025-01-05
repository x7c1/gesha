use crate::testing::v3_0::COMPONENTS_PATH;
use crate::testing::{CanConvert, ConversionError, ConversionSetting, TestCase, TestCasesParent};
use gesha_rust_shapes::ToRustType;
use openapi_types::v3_0;

impl CanConvert<v3_0::ComponentsObject> for gesha_rust_types::SourceCode {
    fn convert(x: v3_0::ComponentsObject) -> Result<Self, ConversionError> {
        ToRustType::apply(x).map_err(ConversionError::RustShape)
    }

    fn test_case_parents() -> Vec<TestCasesParent> {
        vec![schemas(), request_bodies()]
    }
}

impl ConversionSetting<v3_0::ComponentsObject, gesha_rust_types::SourceCode> {
    fn v3_0_rust(parent_name: &str, yaml_name: &str) -> Self {
        let rs_name = yaml_name.replace(".yaml", ".rs");
        Self {
            output: format!("output/v3.0/components/{parent_name}/{rs_name}").into(),
            schema: format!("{COMPONENTS_PATH}/{parent_name}/{yaml_name}").into(),
            example: format!("{COMPONENTS_PATH}/{parent_name}/{rs_name}").into(),
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
    let parent_name = "schemas";
    create_parent(filenames, parent_name)
}

fn request_bodies() -> TestCasesParent {
    let filenames = vec!["schema_ref.yaml"];
    let parent_name = "request_bodies";
    create_parent(filenames, parent_name)
}

fn create_parent(filenames: Vec<&str>, parent_name: &str) -> TestCasesParent {
    let enclosed_cases = filenames
        .iter()
        .map(|filename| TestCase::V3_0_Rust(ConversionSetting::v3_0_rust(parent_name, filename)))
        .collect();

    TestCasesParent {
        file_path: format!("examples/v3_0/src/components/{parent_name}.rs").into(),
        enclosed_cases,
    }
}
