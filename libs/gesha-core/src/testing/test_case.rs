use crate::testing::conversion_setting::ConversionSetting;
use crate::testing::TestCasesParent;
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
        TestCasesParent::all()
            .into_iter()
            .flat_map(|parent| parent.enclosed_cases)
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

    pub fn module_name(&self) -> &str {
        match self {
            TestCase::V3_0_Rust(x) => &x.module_name,
        }
    }
}
