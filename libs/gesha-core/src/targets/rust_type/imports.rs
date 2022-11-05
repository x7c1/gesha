use indexmap::IndexSet;

#[derive(Clone, Debug, Default)]
pub struct Imports(IndexSet<UseStatement>);

impl Imports {
    pub fn new() -> Self {
        Self(IndexSet::new())
    }
    pub fn set(&mut self, x: UseStatement) {
        let _ = self.0.insert(x);
    }
}

impl IntoIterator for Imports {
    type Item = <IndexSet<UseStatement> as IntoIterator>::Item;
    type IntoIter = <IndexSet<UseStatement> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct UseStatement(String);

impl UseStatement {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
}

impl From<UseStatement> for String {
    fn from(x: UseStatement) -> Self {
        x.0
    }
}
