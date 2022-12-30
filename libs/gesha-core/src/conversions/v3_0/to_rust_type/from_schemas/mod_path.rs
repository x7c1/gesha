use openapi_types::v3_0::ComponentName;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ModPath {
    names: Vec<ComponentName>,
}

impl ModPath {
    pub fn new() -> Self {
        ModPath { names: vec![] }
    }

    pub fn add(mut self, name: ComponentName) -> Self {
        self.names.push(name);
        self
    }
}

impl Display for ModPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let path = self
            .names
            .iter()
            .map(|x| x.as_ref())
            .collect::<Vec<&str>>()
            .join("::");

        Display::fmt(&path, f)
    }
}
