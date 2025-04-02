use crate::http::HttpStatusCodeError::{Empty, InvalidChar, LengthExceeded};
use crate::{Error, Result, http};

/// ## OpenAPI v3.1
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.1.md
/// > Status codes SHOULD be selected from the available status codes registered in the IANA Status Code Registry.
///
/// ## OpenAPI v3.0
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#http-status-codes
/// > Status codes SHOULD be selected from the available status codes registered in the IANA Status Code Registry.
///
/// ## IANA HTTP Status Codes
/// https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
/// > The status code of a response is a three-digit integer code
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct HttpStatusCode(String);

impl HttpStatusCode {
    pub fn new(code: String) -> Result<Self> {
        if code.is_empty() {
            return Err(Empty)?;
        }
        if code.len() > 3 {
            return Err(LengthExceeded(code))?;
        }
        if code.chars().any(|c| !c.is_ascii_digit()) {
            return Err(InvalidChar(code))?;
        }
        Ok(Self(code))
    }
}

impl From<HttpStatusCode> for String {
    fn from(code: HttpStatusCode) -> Self {
        code.0
    }
}

impl AsRef<str> for HttpStatusCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HttpStatusCodeError {
    Empty,
    InvalidChar(String),
    LengthExceeded(String),
}

impl From<HttpStatusCodeError> for Error {
    fn from(reason: HttpStatusCodeError) -> Self {
        http::SpecViolation::HttpStatusCode(reason).into()
    }
}
