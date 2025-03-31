use crate::partial_result::{MaybeOps, PartialResult};

pub trait Converter<X, Y, Z> {
    fn convert(self, x: X) -> Z;
}

impl<F, X, Z, E> Converter<PartialResult<X, E>, PartialResult<Z, E>, PartialResult<Z, E>> for F
where
    F: FnOnce(X) -> PartialResult<Z, E>,
{
    fn convert(self, x: PartialResult<X, E>) -> PartialResult<Z, E> {
        x.map(self).flatten()
    }
}

impl<F, X, Z, E> Converter<Result<X, E>, Result<Z, E>, Result<Z, E>> for F
where
    F: FnOnce(X) -> Result<Z, E>,
{
    fn convert(self, x: Result<X, E>) -> Result<Z, E> {
        x.and_then(self)
    }
}

impl<F, X, Z, E> Converter<Result<X, E>, PartialResult<Z, E>, Result<PartialResult<Z, E>, E>> for F
where
    F: FnOnce(X) -> PartialResult<Z, E>,
{
    fn convert(self, x: Result<X, E>) -> Result<PartialResult<Z, E>, E> {
        x.map(self)
    }
}

impl<F, X, Z, E> Converter<Result<Option<X>, E>, PartialResult<Z, E>, PartialResult<Option<Z>, E>>
    for F
where
    F: FnOnce(X) -> PartialResult<Z, E>,
{
    fn convert(self, x: Result<Option<X>, E>) -> PartialResult<Option<Z>, E> {
        x.maybe().map(|maybe_a| maybe_a.map(self).maybe()).flatten()
    }
}

impl<F, X, Z, E> Converter<Result<Option<X>, E>, Result<Z, E>, PartialResult<Option<Z>, E>> for F
where
    F: FnOnce(X) -> Result<Z, E>,
{
    fn convert(self, x: Result<Option<X>, E>) -> PartialResult<Option<Z>, E> {
        x.maybe().map(|maybe_a| maybe_a.map(self).maybe()).flatten()
    }
}

impl<F, X, Z, E>
    Converter<Result<Option<X>, E>, Result<PartialResult<Z, E>, E>, PartialResult<Option<Z>, E>>
    for F
where
    F: FnOnce(X) -> Result<PartialResult<Z, E>, E>,
{
    fn convert(self, result: Result<Option<X>, E>) -> PartialResult<Option<Z>, E> {
        let x = match result {
            Ok(Some(x)) => x,
            Ok(None) => return PartialResult::new(None, vec![]),
            Err(e) => return PartialResult::new(None, vec![e]),
        };
        let partial = match self(x) {
            Ok(partial) => partial,
            Err(e) => return PartialResult::new(None, vec![e]),
        };
        partial.map(Some)
    }
}
