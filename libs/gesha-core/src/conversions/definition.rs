use crate::conversions;
use crate::conversions::{TestCase, TestSuite};
use crate::Error::UnknownTestCase;
use openapi_types::yaml::ToOpenApi;
use std::fmt::Display;

pub trait Definition: Sized + Send + Sync + 'static {
    type OpenApiType: ToOpenApi + Send + Sync;
    type TargetType: Display + Send + Sync;

    fn convert(x: Self::OpenApiType) -> Result<Self::TargetType, conversions::Error>;

    fn test_suites() -> Vec<TestSuite<Self>>;

    fn test_suites_content(suite: &TestSuite<Self>) -> impl Display;

    fn list_test_cases() -> Vec<TestCase<Self>> {
        Self::test_suites()
            .into_iter()
            .flat_map(|suite| suite.test_cases)
            .collect()
    }

    fn require_test_case(path: &str) -> Result<TestCase<Self>, crate::Error> {
        Self::list_test_cases()
            .into_iter()
            .find(|case| case.schema.as_os_str() == path)
            .ok_or_else(|| UnknownTestCase {
                path: path.to_string(),
            })
    }
}
