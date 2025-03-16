use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModuleName(String);

impl ModuleName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
    pub fn append(mut self, a: &str) -> Self {
        self.0.push_str(a);
        self
    }
}

impl Display for ModuleName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<ModuleName> for String {
    fn from(this: ModuleName) -> Self {
        this.0
    }
}

impl AsRef<str> for ModuleName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
