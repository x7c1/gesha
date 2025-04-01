pub use crate::Result;

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
#[derive(Debug)]
pub struct HttpStatusCode(String);

impl HttpStatusCode {
    pub fn new(code: String) -> Result<Self> {
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
