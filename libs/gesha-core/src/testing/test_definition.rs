use crate::conversions::Converter;
use crate::testing::Error::UnknownTestCase;
use crate::testing::{TestCase, TestCaseIndex};

pub trait TestDefinition: Converter {
    /// List all test cases for this definition.
    fn list_indexes(&self) -> Vec<TestCaseIndex<Self>>;

    /// Generate the code of the test case index.
    fn generate_index_code(&self, index: &TestCaseIndex<Self>) -> Self::TargetType;

    /// List all test cases for this definition.
    fn list_test_cases(&self) -> Vec<TestCase<Self>> {
        self.list_indexes()
            .into_iter()
            .flat_map(|index| index.test_cases)
            .collect()
    }

    /// Get a test case by its path.
    /// If the test case does not exist, return an error.
    fn require_test_case(&self, path: &str) -> Result<TestCase<Self>, crate::Error> {
        let case = self
            .list_test_cases()
            .into_iter()
            .find(|case| case.schema.as_os_str() == path)
            .ok_or_else(|| UnknownTestCase {
                path: path.to_string(),
            })?;

        Ok(case)
    }
}
