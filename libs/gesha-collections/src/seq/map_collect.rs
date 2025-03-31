use crate::partial_result::{MergeOps, PartialResult};

pub trait MapCollect<A, E> {
    fn map_collect<B>(self, f: impl FnMut(A) -> Result<B, E>) -> PartialResult<Vec<B>, E>;
}

impl<A, E> MapCollect<A, E> for Vec<A> {
    fn map_collect<B>(self, f: impl FnMut(A) -> Result<B, E>) -> PartialResult<Vec<B>, E> {
        self.into_iter()
            .map(f)
            .collect::<Vec<Result<B, E>>>()
            .merge()
    }
}
