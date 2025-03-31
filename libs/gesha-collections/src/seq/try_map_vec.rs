pub trait TryMapVec<A> {
    fn try_map<B, E>(self, f: impl FnMut(A) -> Result<B, E>) -> Result<Vec<B>, E>;
}

impl<A> TryMapVec<A> for Vec<A> {
    fn try_map<B, E>(self, f: impl FnMut(A) -> Result<B, E>) -> Result<Vec<B>, E> {
        self.into_iter().map(f).collect()
    }
}
