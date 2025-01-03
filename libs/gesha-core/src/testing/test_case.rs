use crate::testing::conversion_setting::ConversionSetting;
use crate::Result;
use openapi_types::v3_0;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TestCase {
    /// conversion from OpenAPI v3.0.* to Rust codes
    V3_0_Rust(ConversionSetting<v3_0::ComponentsObject, gesha_rust_types::SourceCode>),
}

impl TestCase {
    pub fn all() -> Vec<Self> {
        vec![Self::V3_0_Rust(ConversionSetting::v3_0_rust("array.yaml"))]
    }
    pub fn require(_path: &str) -> Result<Self> {
        todo!()
    }
}
