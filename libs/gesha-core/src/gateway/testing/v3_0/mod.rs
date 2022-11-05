mod component_cases;
pub use component_cases::{ComponentCase, ComponentCases};

#[derive(Clone, Copy)]
pub enum ComponentKind {
    RequestBodies,
    Schemas,
}

impl ComponentKind {
    pub fn name(&self) -> &str {
        match self {
            ComponentKind::RequestBodies => "request_bodies",
            ComponentKind::Schemas => "schemas",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::RequestBodies, Self::Schemas]
    }
}
