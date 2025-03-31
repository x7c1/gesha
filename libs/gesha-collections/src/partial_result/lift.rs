use crate::partial_result::PartialResult;

pub trait LiftOps<A, B, E> {
    fn lift(self) -> PartialResult<(A, B), E>;
}

impl<A, B, E> LiftOps<A, B, E> for (A, PartialResult<B, E>) {
    fn lift(self) -> PartialResult<(A, B), E> {
        let (a, PartialResult(b, errors)) = self;
        PartialResult((a, b), errors)
    }
}
