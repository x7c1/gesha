use crate::testing::conversion_setting::ConversionSetting;
use crate::Error::UnknownTestCase;
use crate::Result;
use openapi_types::v3_0;
use std::path::Path;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TestCase {
    /// conversion from OpenAPI v3.0.* to Rust codes
    V3_0_Rust(ConversionSetting<v3_0::ComponentsObject, gesha_rust_types::SourceCode>),
}

impl TestCase {
    pub fn all() -> Vec<Self> {
        let cases = vec![
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
        cases
            .iter()
            .map(|filename| Self::V3_0_Rust(ConversionSetting::v3_0_rust(filename)))
            .collect()
    }

    pub fn require(path: &str) -> Result<Self> {
        Self::all()
            .into_iter()
            .find(|case| case.schema_path().as_os_str() == path)
            .ok_or(UnknownTestCase {
                path: path.to_string(),
            })
    }

    pub fn schema_path(&self) -> &Path {
        match self {
            TestCase::V3_0_Rust(x) => &x.schema,
        }
    }
}
