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

    pub fn depth(&self) -> usize {
        self.names.len()
    }

    pub fn relative_from(&self, target: Self) -> Self {
        if self.names.starts_with(&target.names) {
            let (_, tail) = self.names.split_at(target.names.len());
            return tail.to_vec().into();
        }
        vec!["super"]
            .repeat(target.depth())
            .into_iter()
            .map(ComponentName::new)
            .chain(self.names.clone())
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<Vec<ComponentName>> for ModPath {
    fn from(names: Vec<ComponentName>) -> Self {
        ModPath { names }
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
