use std::borrow::Borrow;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum SerdeAttribute {
    Untagged,
}

impl Borrow<str> for SerdeAttribute {
    fn borrow(&self) -> &str {
        match self {
            SerdeAttribute::Untagged => "untagged",
        }
    }
}
