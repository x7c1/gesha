use openapi_types::core::{Output, OutputMergeOps};

pub trait TryMap<A> {
    fn try_map<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Vec<B>, E>;
}

impl<A> TryMap<A> for Vec<A> {
    fn try_map<B, E>(self, f: impl Fn(A) -> Result<B, E>) -> Result<Vec<B>, E> {
        self.into_iter().map(f).collect()
    }
}

pub trait OutputResult<A, E> {
    fn map_each<B>(self, f: impl FnMut(A) -> Result<B, E>) -> Output<Vec<B>, E>;
}

impl<A, E> OutputResult<A, E> for Vec<A> {
    fn map_each<B>(self, f: impl FnMut(A) -> Result<B, E>) -> Output<Vec<B>, E> {
        self.into_iter()
            .map(f)
            .collect::<Vec<Result<B, E>>>()
            .merge()
    }
}
