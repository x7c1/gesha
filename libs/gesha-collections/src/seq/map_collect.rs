use crate::partial_result::{MergeOps, PartialResult};
use crate::seq::TryMapOps;

pub trait MapCollectOps<A, E> {
    fn map_collect<B>(self, f: impl FnMut(A) -> Result<B, E>) -> PartialResult<Vec<B>, E>;
}

impl<A, E> MapCollectOps<A, E> for Vec<A> {
    fn map_collect<B>(self, f: impl FnMut(A) -> Result<B, E>) -> PartialResult<Vec<B>, E> {
        self.try_map(f).merge()
    }
}
