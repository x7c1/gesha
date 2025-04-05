use crate::partial_result::PartialResult;

pub trait WithContextOps<C> {
    fn with_context(self, context: impl Into<C>) -> Self;
}

impl<A, E, C> WithContextOps<C> for Result<A, E>
where
    E: ContextAppendable<C>,
{
    fn with_context(self, context: impl Into<C>) -> Self {
        self.map_err(|x| E::append(context, x))
    }
}

impl<A, E, C> WithContextOps<C> for PartialResult<A, E>
where
    E: ContextBindable<C>,
{
    fn with_context(self, context: impl Into<C>) -> Self {
        self.bind_errors(|xs| E::bind(context, xs))
    }
}

pub trait ContextAppendable<C>: Sized {
    fn append(context: impl Into<C>, error: Self) -> Self;
}

pub trait ContextBindable<C>: Sized {
    fn bind(context: impl Into<C>, error: Vec<Self>) -> Self;
}
