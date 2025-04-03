use crate::partial_result::PartialResult;

pub trait WithKeyOps {
    fn with_key(self, key: impl Into<String>) -> Self;
}

impl<A, E> WithKeyOps for Result<A, E>
where
    E: KeyAppendable,
{
    fn with_key(self, key: impl Into<String>) -> Self {
        self.map_err(|x| E::append_key(key, x))
    }
}

impl<A, E> WithKeyOps for PartialResult<A, E>
where
    E: KeyBindable,
{
    fn with_key(self, key: impl Into<String>) -> Self {
        self.bind_errors(|xs| E::bind_key(key, xs))
    }
}

pub trait KeyAppendable {
    fn append_key(key: impl Into<String>, error: Self) -> Self;
}

pub trait KeyBindable: Sized {
    fn bind_key(key: impl Into<String>, error: Vec<Self>) -> Self;
}
