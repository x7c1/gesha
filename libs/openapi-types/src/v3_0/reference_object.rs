/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#referenceObject
#[derive(Debug)]
pub struct ReferenceObject(String);

impl ReferenceObject {
    pub fn new<A: Into<String>>(a: A) -> Self {
        ReferenceObject(a.into())
    }
}

impl From<ReferenceObject> for String {
    fn from(this: ReferenceObject) -> Self {
        this.0
    }
}
