use std::fmt::{Display, Formatter};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#dataTypes
#[derive(Debug)]
pub enum FormatModifier {
    Int32,
    Int64,
    Float,
    Double,

    /// > the format property is an open string-valued property,
    /// > and can have any value. Formats such as "email", "uuid", and so on,
    /// > MAY be used even though undefined by this specification.
    Custom(String),
    /*
    // TODO: support formats defined by the OAS:
    Byte,
    Binary,
    Date,
    DateTime,
    Password,
    */
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
            Self::Int32 => "int32",
            Self::Int64 => "int64",
            Self::Float => "float",
            Self::Double => "double",
            Self::Custom(x) => x,
        }
    }
}

impl Display for FormatModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{str}", str = self.as_ref())
    }
}
