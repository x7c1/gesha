#[derive(Debug, Clone, PartialEq)]
pub enum SpecViolation {
    FieldNotExist {
        field: String,
    },

    /// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#data-types
    UnknownDataType {
        found: String,
    },

    TypeMismatch {
        expected: String,
        found: String,
    },
}

impl From<SpecViolation> for crate::SpecViolation {
    fn from(reason: SpecViolation) -> Self {
        crate::SpecViolation::V3_0(reason)
    }
}

impl From<SpecViolation> for crate::Error {
    fn from(reason: SpecViolation) -> Self {
        crate::Error::SpecViolation(crate::SpecViolation::V3_0(reason))
    }
}
