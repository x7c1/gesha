use crate::Definition;
use gesha_collections::seq::TryMapOps;
use gesha_core::conversions::Result;
use gesha_core::{broken_defs, conversions};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Definitions(Vec<Definition>);

impl Definitions {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return `gesha_core::conversions::Error::TransformBroken`
    /// if a definition with the same name has already been pushed.
    pub fn set<A: Into<Definition>>(&mut self, def: A) -> Result<()> {
        let def = def.into();
        let name = def.symbol_name();

        if self.already_pushed(def.symbol_name()) {
            return Err(broken_defs!(name.to_string()));
        }
        self.0.push(def);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Definition> {
        self.0.iter()
    }

    pub fn from<A>(xs: Vec<A>) -> Result<Self>
    where
        A: TryInto<Definition, Error = conversions::Error>,
    {
        xs.try_map(|x| x.try_into()).map(Self)
    }

    fn already_pushed(&self, name: &str) -> bool {
        self.0.iter().any(|x| x.symbol_name() == name)
    }
}

impl FromIterator<Definition> for Definitions {
    fn from_iter<T: IntoIterator<Item = Definition>>(iter: T) -> Self {
        let set = iter.into_iter().collect();
        Self(set)
    }
}

impl IntoIterator for Definitions {
    type Item = <Vec<Definition> as IntoIterator>::Item;
    type IntoIter = <Vec<Definition> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}
