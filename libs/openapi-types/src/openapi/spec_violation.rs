#[derive(Debug, Clone, PartialEq)]
pub enum SpecViolation {
    FieldNotExist { field: String },
    UnknownDataType { found: String },
    TypeMismatch { expected: String, found: String },
}

impl From<SpecViolation> for crate::Error {
    fn from(reason: SpecViolation) -> Self {
        crate::Error::SpecViolation(crate::SpecViolation::OpenApi(reason))
    }
}
