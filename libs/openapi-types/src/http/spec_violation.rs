use crate::Error;
use crate::http::HttpStatusCodeError;

#[derive(Debug, Clone, PartialEq)]
pub enum SpecViolation {
    HttpStatusCode(HttpStatusCodeError),
}

impl From<SpecViolation> for Error {
    fn from(this: SpecViolation) -> Self {
        Error::SpecViolation(crate::SpecViolation::Http(this))
    }
}
