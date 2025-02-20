use crate::conversions;
use crate::conversions::{TestCase, TestSuite};
use crate::Error::UnknownTestCase;
use openapi_types::yaml::ToOpenApi;
use std::fmt::Display;
use std::path::Path;

/// A definition of a conversion.
pub trait Definition: Sized + Send + Sync + 'static {
    /// The OpenAPI type that this definition converts from.
    type OpenApiType: ToOpenApi + Send + Sync;

    /// The target type that this definition converts to.
    type TargetType: Display + Send + Sync;

    /// Convert the given OpenAPI type to the target type.
    fn convert(x: Self::OpenApiType) -> Result<Self::TargetType, conversions::Error>;

    /// Format the code in the given path.
    fn format_code(path: &Path) -> crate::Result<String>;
}

pub trait TestDefinition: Definition {
    /// List all test suites for this definition.
    fn test_suites() -> Vec<TestSuite<Self>>;

    /// Get the code of the test suite.
    fn test_suite_code(suite: &TestSuite<Self>) -> Self::TargetType;

    /// List all test cases for this definition.
    fn list_test_cases() -> Vec<TestCase<Self>> {
        Self::test_suites()
            .into_iter()
            .flat_map(|suite| suite.test_cases)
            .collect()
    }

    /// Get a test case by its path.
    /// If the test case does not exist, return an error.
    fn require_test_case(path: &str) -> Result<TestCase<Self>, crate::Error> {
        Self::list_test_cases()
            .into_iter()
            .find(|case| case.schema.as_os_str() == path)
            .ok_or_else(|| UnknownTestCase {
                path: path.to_string(),
            })
    }
}
