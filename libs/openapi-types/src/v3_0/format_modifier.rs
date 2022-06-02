use std::fmt::{Display, Formatter};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#dataTypes
#[derive(Debug)]
pub enum FormatModifier {
    Int32,
    Int64,
    Float,
    Double,
    // TODO:
    // > the format property is an open string-valued property,
    // > and can have any value. Formats such as "email", "uuid", and so on,
    // > MAY be used even though undefined by this specification.
}

impl FormatModifier {
    pub fn find(target: &str) -> Option<FormatModifier> {
        [Self::Int32, Self::Int64, Self::Float, Self::Double]
            .into_iter()
            .find(|x| x.as_ref() == target)
    }
}

impl AsRef<str> for FormatModifier {
    fn as_ref(&self) -> &str {
        match self {
            FormatModifier::Int32 => "int32",
            FormatModifier::Int64 => "int64",
            FormatModifier::Float => "float",
            FormatModifier::Double => "double",
        }
    }
}

impl Display for FormatModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{str}", str = self.as_ref())
    }
}
