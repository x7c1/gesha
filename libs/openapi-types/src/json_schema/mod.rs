#[derive(Debug, Clone, PartialEq)]
pub enum SpecViolation {
    /// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.15
    /// > This array MUST have at least one element
    EmptyRequiredField,
}

impl From<SpecViolation> for crate::SpecViolation {
    fn from(reason: SpecViolation) -> Self {
        crate::SpecViolation::JsonSchema(reason)
    }
}

impl From<SpecViolation> for crate::Error {
    fn from(reason: SpecViolation) -> Self {
        crate::Error::SpecViolation(reason.into())
    }
}
