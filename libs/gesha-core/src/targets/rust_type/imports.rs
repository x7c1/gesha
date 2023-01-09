use indexmap::IndexSet;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use Cow::{Borrowed, Owned};

#[derive(Clone, Debug, PartialEq, Default)]
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
    Patch { depth: usize },
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Self::Deserialize => Borrowed("serde::Deserialize"),
            Self::Deserializer => Borrowed("serde::Deserializer"),
            Self::Serialize => Borrowed("serde::Serialize"),
            Self::Serializer => Borrowed("serde::Serializer"),
            Self::Display => Borrowed("std::fmt::Display"),
            Self::Formatter => Borrowed("std::fmt::Formatter"),
            Self::Patch { depth } => Owned("super::".repeat(*depth) + "core::Patch"),
        };
        Display::fmt(&x, f)
    }
}

impl From<Package> for Vec<Package> {
    fn from(x: Package) -> Self {
        vec![x]
    }
}
