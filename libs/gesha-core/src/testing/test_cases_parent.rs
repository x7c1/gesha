use crate::testing::{CanConvert, TestCase};
use openapi_types::v3_0;
use std::path::PathBuf;

pub struct TestCasesParent {
    pub file_path: PathBuf,
    pub enclosed_cases: Vec<TestCase>,
}

impl TestCasesParent {
    pub fn all() -> Vec<Self> {
        <gesha_rust_types::SourceCode as CanConvert<v3_0::ComponentsObject>>::test_case_parents()
    }
}
