use crate::conversions::TestCase;
use std::path::PathBuf;

pub struct TestSuite<A> {
    pub mod_path: PathBuf,
    pub test_cases: Vec<TestCase<A>>,
}
