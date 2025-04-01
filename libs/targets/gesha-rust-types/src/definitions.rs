use crate::{Definition, Error};
use Error::DefinitionAlreadyExists;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Definitions(Vec<Definition>);

impl Definitions {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return `DefinitionAlreadyExists`
    /// if a definition with the same name has already been pushed.
    pub fn set<A: Into<Definition>>(&mut self, def: A) -> crate::Result<()> {
        let def = def.into();
        let name = def.name();

        if self.already_pushed(def.name()) {
            return Err(DefinitionAlreadyExists {
                name: name.to_string(),
            });
        }
        self.0.push(def);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Definition> {
        self.0.iter()
    }

    pub fn from<A, E>(xs: Vec<A>) -> Result<Self, E>
    where
        A: TryInto<Definition, Error = E>,
    {
        xs.into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<_>, E>>()
            .map(Self)
    }

    fn already_pushed(&self, name: &str) -> bool {
        self.0.iter().any(|x| x.name() == name)
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
