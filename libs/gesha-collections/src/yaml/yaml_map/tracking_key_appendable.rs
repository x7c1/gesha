use crate::partial_result::PartialResult;

pub trait TrackingKeyAppendable {
    fn append(self, key: &str) -> Self;
}

impl<A, E> TrackingKeyAppendable for Result<A, E>
where
    E: KeyAppendable,
{
    fn append(self, key: &str) -> Self {
        self.map_err(|x| E::append_key(key, x))
    }
}

impl<A, E> TrackingKeyAppendable for PartialResult<A, E>
where
    E: KeyBindable,
{
    fn append(self, key: &str) -> Self {
        self.bind_errors(|xs| E::bind_key(key, xs))
    }
}

pub trait KeyAppendable {
    fn append_key(key: &str, error: Self) -> Self;
}

pub trait KeyBindable: Sized {
    fn bind_key(key: &str, error: Vec<Self>) -> Self;
}
