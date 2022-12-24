use indexmap::IndexSet;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default)]
pub struct Imports(IndexSet<Package>);

impl Imports {
    pub fn new() -> Self {
        Self(IndexSet::new())
    }
    pub fn set<A: Into<Vec<Package>>>(&mut self, xs: A) {
        xs.into().into_iter().for_each(|x| {
            let _ = self.0.insert(x);
        })
    }
}

impl IntoIterator for Imports {
    type Item = <IndexSet<Package> as IntoIterator>::Item;
    type IntoIter = <IndexSet<Package> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

impl From<Vec<Package>> for Imports {
    fn from(this: Vec<Package>) -> Self {
        let mut xs = Self::new();
        xs.set(this);
        xs
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Package {
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
    Display,
    Formatter,
    Patch,
}

impl AsRef<str> for Package {
    fn as_ref(&self) -> &str {
        match self {
            Self::Deserialize => "serde::Deserialize",
            Self::Deserializer => "serde::Deserializer",
            Self::Serialize => "serde::Serialize",
            Self::Serializer => "serde::Serializer",
            Self::Display => "std::fmt::Display",
            Self::Formatter => "std::fmt::Formatter",
            Self::Patch => "super::core::Patch",
        }
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_ref(), f)
    }
}

impl From<Package> for Vec<Package> {
    fn from(x: Package) -> Self {
        vec![x]
    }
}
