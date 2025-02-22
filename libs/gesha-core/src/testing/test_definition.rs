use crate::conversions::Converter;
use crate::testing::{TestCase, TestCaseIndex};
use crate::Error::UnknownTestCase;

pub trait TestDefinition: Converter {
    /// List all test cases for this definition.
    fn test_indexes(&self) -> Vec<TestCaseIndex<Self>>;

    /// Get the code of the test index.
    fn test_index_code(&self, index: &TestCaseIndex<Self>) -> Self::TargetType;

    /// List all test cases for this definition.
    fn list_test_cases(&self) -> Vec<TestCase<Self>> {
        self.test_indexes()
            .into_iter()
            .flat_map(|index| index.test_cases)
            .collect()
    }

    /// Get a test case by its path.
    /// If the test case does not exist, return an error.
    fn require_test_case(&self, path: &str) -> Result<TestCase<Self>, crate::Error> {
        self.list_test_cases()
            .into_iter()
            .find(|case| case.schema.as_os_str() == path)
            .ok_or_else(|| UnknownTestCase {
                path: path.to_string(),
            })
    }
}
