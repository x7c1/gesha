use crate::conversion;
use crate::conversion::{TestCase, TestSuite};
use crate::Error::UnknownTestCase;
use std::fmt::Display;

pub trait Definition {
    type OpenApiType;
    type TargetType;

    fn convert(x: Self::OpenApiType) -> Result<Self::TargetType, conversion::Error>;

    fn test_suites() -> Vec<TestSuite<Self::OpenApiType, Self::TargetType>>;

    fn test_suites_content(suite: &TestSuite<Self::OpenApiType, Self::TargetType>) -> impl Display;

    fn list_test_cases() -> Vec<TestCase<Self::OpenApiType, Self::TargetType>> {
        Self::test_suites()
            .into_iter()
            .flat_map(|suite| suite.test_cases)
            .collect()
    }

    fn require_test_case(
        path: &str,
    ) -> Result<TestCase<Self::OpenApiType, Self::TargetType>, crate::Error> {
        Self::list_test_cases()
            .into_iter()
            .find(|case| case.schema.as_os_str() == path)
            .ok_or_else(|| UnknownTestCase {
                path: path.to_string(),
            })
    }
}
