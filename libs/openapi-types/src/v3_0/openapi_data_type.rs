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
