use gesha_rust_types::DataType;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct TypePath {
    names: Vec<String>,
}

impl TypePath {
    pub fn new() -> Self {
        TypePath { names: vec![] }
    }

    pub fn add<A: Into<String>>(mut self, name: A) -> Self {
        self.names.push(name.into());
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
        target
            .to_supers()
            .chain(self.names.clone())
            .collect::<Vec<_>>()
            .into()
    }

    pub fn ancestors(&self) -> Self {
        self.to_supers().collect::<Vec<_>>().into()
    }

    fn to_supers(&self) -> impl Iterator<Item = String> {
        vec!["super"]
            .repeat(self.depth())
            .into_iter()
            .map(String::from)
    }
}

impl Default for TypePath {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<String>> for TypePath {
    fn from(names: Vec<String>) -> Self {
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
