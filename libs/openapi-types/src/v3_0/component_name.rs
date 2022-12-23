use std::fmt::{Display, Formatter};
use heck::ToUpperCamelCase;

/// > All the fixed fields declared above are objects
/// > that MUST use keys that match the regular expression: ^[a-zA-Z0-9\.\-_]+$.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ComponentName(String);

impl ComponentName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        ComponentName(a.into())
    }
    pub fn to_upper_camel_case(&self) -> Self {
        Self(self.0.to_upper_camel_case())
    }
}

impl From<ComponentName> for String {
    fn from(this: ComponentName) -> Self {
        this.0
    }
}

impl AsRef<str> for ComponentName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for ComponentName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
