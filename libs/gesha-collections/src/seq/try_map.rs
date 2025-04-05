pub trait TryMapOps<A> {
    fn try_map<B, E>(self, f: impl FnMut(A) -> Result<B, E>) -> Result<Vec<B>, E>;
}

impl<I, A> TryMapOps<A> for I
where
    I: IntoIterator<Item = A>,
{
    fn try_map<B, E>(self, f: impl FnMut(A) -> Result<B, E>) -> Result<Vec<B>, E> {
        self.into_iter().map(f).collect()
    }
}
