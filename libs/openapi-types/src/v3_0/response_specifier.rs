use crate::Result;
use crate::http::HttpStatusCode;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#responses-object
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ResponseSpecifier {
    /// > The default MAY be used as a default Response Object
    /// > for all HTTP codes that are not covered individually by the Responses Object.
    Default,
    HttpStatusCode(HttpStatusCode),
}

impl ResponseSpecifier {
    pub fn from_string(value: String) -> Result<Self> {
        if value == "default" {
            return Ok(Self::Default);
        }
        let code = HttpStatusCode::new(value)?;
        Ok(Self::HttpStatusCode(code))
    }
}
