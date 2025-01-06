use crate::conversions::TestCase;
use std::path::PathBuf;

pub struct TestSuite<From, To> {
    pub mod_path: PathBuf,
    pub test_cases: Vec<TestCase<From, To>>,
}
