use crate::ModuleName;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct ModuleDeclarations(Vec<ModuleName>);

impl Display for ModuleDeclarations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .try_for_each(|module| writeln!(f, "pub mod {};", module))
    }
}

impl FromIterator<ModuleName> for ModuleDeclarations {
    fn from_iter<T: IntoIterator<Item = ModuleName>>(iter: T) -> Self {
        let set = iter.into_iter().collect();
        Self(set)
    }
}
