use crate::targets::rust_type::DataType;
use openapi_types::v3_0::ComponentName;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct TypePath {
    names: Vec<ComponentName>,
}

impl TypePath {
    pub fn new() -> Self {
        TypePath { names: vec![] }
    }

    pub fn add<A: Into<String>>(mut self, name: A) -> Self {
        let name = ComponentName::new(name);
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

    pub fn ancestors(&self) -> Self {
        vec!["super"]
            .repeat(self.depth())
            .into_iter()
            .map(ComponentName::new)
            .collect::<Vec<_>>()
            .into()
    }
}

impl Default for TypePath {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<ComponentName>> for TypePath {
    fn from(names: Vec<ComponentName>) -> Self {
        TypePath { names }
    }
}

impl From<TypePath> for DataType {
    fn from(this: TypePath) -> Self {
        DataType::Custom(this.to_string())
    }
}

impl Display for TypePath {
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
