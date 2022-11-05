use crate::targets::rust_type::Definition;
use indexmap::IndexSet;

#[derive(Clone, Debug, Default)]
pub struct Definitions(IndexSet<Definition>);

impl Definitions {
    pub fn new() -> Self {
        Self(IndexSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn set(&mut self, x: Definition) {
        let _ = self.0.insert(x);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Definition> {
        self.0.iter()
    }
}

impl FromIterator<Definition> for Definitions {
    fn from_iter<T: IntoIterator<Item = Definition>>(iter: T) -> Self {
        let set = iter.into_iter().collect();
        Self(set)
    }
}

impl IntoIterator for Definitions {
    type Item = <IndexSet<Definition> as IntoIterator>::Item;
    type IntoIter = <IndexSet<Definition> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}
