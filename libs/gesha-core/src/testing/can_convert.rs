use crate::testing::TestCasesParent;

pub trait CanConvert<From>: Sized {
    fn convert(x: From) -> Result<Self, ConversionError>;
    fn test_case_parents() -> Vec<TestCasesParent>;
}

#[derive(Debug)]
pub enum ConversionError {
    RustShape(gesha_rust_shapes::Error),
}
