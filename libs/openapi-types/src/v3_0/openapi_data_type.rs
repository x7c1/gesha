use crate::Result;
use crate::v3_0::SpecViolation::UnknownDataType;
use std::fmt::{Display, Formatter};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#dataTypes
#[derive(Clone, Debug)]
pub enum OpenApiDataType {
    Array,
    Boolean,
    Integer,
    Number,
    Object,
    String,
}

impl OpenApiDataType {
    pub fn new(found: String) -> Result<Self> {
        Self::find(&found).ok_or_else(|| UnknownDataType { found }.into())
    }
    pub fn find(target: &str) -> Option<OpenApiDataType> {
        [
            Self::Array,
            Self::Boolean,
            Self::Integer,
            Self::Number,
            Self::Object,
            Self::String,
        ]
        .into_iter()
        .find(|x| x.as_ref() == target)
    }
}

impl AsRef<str> for OpenApiDataType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Array => "array",
            Self::Boolean => "boolean",
            Self::Integer => "integer",
            Self::Number => "number",
            Self::Object => "object",
            Self::String => "string",
        }
    }
}

impl Display for OpenApiDataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_ref(), f)
    }
}
