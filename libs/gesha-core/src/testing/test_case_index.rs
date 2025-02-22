use crate::testing::TestCase;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TestCaseIndex<A> {
    pub mod_path: PathBuf,
    pub test_cases: Vec<TestCase<A>>,
}
