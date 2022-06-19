use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum DeriveAttribute {
    Clone,
    Debug,
    Deserialize,
    PartialEq,
    Serialize,
}

impl DeriveAttribute {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Clone,
            Self::Debug,
            Self::Deserialize,
            Self::PartialEq,
            Self::Serialize,
        ]
    }
}

impl Borrow<str> for DeriveAttribute {
    fn borrow(&self) -> &str {
        match self {
            Self::Clone => "Clone",
            Self::Debug => "Debug",
            Self::Deserialize => "Deserialize",
            Self::PartialEq => "PartialEq",
            Self::Serialize => "Serialize",
        }
    }
}

impl Display for DeriveAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x: &str = Borrow::borrow(self);
        Display::fmt(x, f)
    }
}
