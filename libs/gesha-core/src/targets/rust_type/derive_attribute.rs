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
