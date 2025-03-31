use crate::partial_result::PartialResult;

pub trait MaybeOps<A, E> {
    fn maybe(self) -> PartialResult<Option<A>, E>;
}

impl<A, E> MaybeOps<A, E> for Option<PartialResult<A, E>> {
    fn maybe(self) -> PartialResult<Option<A>, E> {
        match self {
            None => PartialResult(None, vec![]),
            Some(PartialResult(a, errors)) => PartialResult(Some(a), errors),
        }
    }
}

impl<A, E> MaybeOps<A, E> for Option<Result<A, E>> {
    fn maybe(self) -> PartialResult<Option<A>, E> {
        match self {
            None => PartialResult(None, vec![]),
            Some(Ok(a)) => PartialResult(Some(a), vec![]),
            Some(Err(e)) => PartialResult(None, vec![e]),
        }
    }
}

impl<A, E> MaybeOps<A, E> for Result<Option<A>, E> {
    fn maybe(self) -> PartialResult<Option<A>, E> {
        match self {
            Ok(Some(a)) => PartialResult(Some(a), vec![]),
            Ok(None) => PartialResult(None, vec![]),
            Err(e) => PartialResult(None, vec![e]),
        }
    }
}

impl<A, E> MaybeOps<A, E> for Result<A, E> {
    fn maybe(self) -> PartialResult<Option<A>, E> {
        match self {
            Ok(a) => PartialResult(Some(a), vec![]),
            Err(e) => PartialResult(None, vec![e]),
        }
    }
}

impl<A, E> MaybeOps<A, E> for PartialResult<Result<A, E>, E> {
    fn maybe(self) -> PartialResult<Option<A>, E> {
        match self {
            PartialResult(Ok(a), errors) => PartialResult(Some(a), errors),
            PartialResult(Err(e), mut errors) => {
                errors.push(e);
                PartialResult(None, errors)
            }
        }
    }
}
