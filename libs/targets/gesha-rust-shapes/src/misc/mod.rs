use gesha_collections::partial_result::{MergeOps, PartialResult};

pub trait MapOutput<A, E> {
    fn map_output<B>(self, f: impl FnMut(A) -> Result<B, E>) -> PartialResult<Vec<B>, E>;
}

impl<A, E> MapOutput<A, E> for Vec<A> {
    fn map_output<B>(self, f: impl FnMut(A) -> Result<B, E>) -> PartialResult<Vec<B>, E> {
        self.into_iter()
            .map(f)
            .collect::<Vec<Result<B, E>>>()
            .merge()
    }
}
