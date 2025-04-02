use crate::v3_0::{PathFieldName, ResponseSpecifier};

#[derive(Debug, Clone, PartialEq)]
pub enum SpecViolation {
    DuplicatedPathFieldName {
        fields: Vec<PathFieldName>,
    },
    DuplicatedResponseSpecifier {
        fields: Vec<ResponseSpecifier>,
    },

    /// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#responses-object
    /// > The Responses Object MUST contain at least one response code,
    EmptyResponses,

    /// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#paths-object
    InvalidPathFieldName {
        field: String,
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
