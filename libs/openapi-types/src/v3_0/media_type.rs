/// > The key is a media type or media type range and the value describes it.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct MediaTypeKey(String);

impl MediaTypeKey {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
}

#[derive(Clone, Debug)]
pub struct MediaTypeObject {}
