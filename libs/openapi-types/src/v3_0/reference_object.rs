use std::marker::PhantomData;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#referenceObject
#[derive(Clone, Debug)]
pub struct ReferenceObject<T>(String, PhantomData<T>);

impl<T> ReferenceObject<T> {
    pub fn new<A: Into<String>>(a: A) -> Self {
        ReferenceObject(a.into(), PhantomData::default())
    }
}

impl<T> From<ReferenceObject<T>> for String {
    fn from(this: ReferenceObject<T>) -> Self {
        this.0
    }
}

impl<T> AsRef<str> for ReferenceObject<T> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
