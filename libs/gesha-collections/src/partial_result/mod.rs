mod lift;
pub use lift::LiftOps;

mod maybe;
pub use maybe::MaybeOps;

mod merge;
pub use merge::MergeOps;

#[derive(Debug)]
pub struct PartialResult<A, E>(pub(crate) A, pub(crate) Vec<E>);

impl<A, E> PartialResult<A, E> {
    pub fn new(a: A, errors: Vec<E>) -> Self {
        PartialResult(a, errors)
    }

    pub fn ok(a: A) -> Self {
        PartialResult(a, vec![])
    }

    pub fn append(mut self, errors: Vec<E>) -> Self {
        self.1.extend(errors);
        self
    }

    pub fn into_tuple(self) -> (A, Vec<E>) {
        (self.0, self.1)
    }

    pub fn map<B, F>(self, f: F) -> PartialResult<B, E>
    where
        F: FnOnce(A) -> B,
    {
        let Self(a, errors) = self;
        let b = f(a);
        PartialResult(b, errors)
    }

    pub fn bind_errors<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Vec<E>) -> E,
    {
        if self.1.is_empty() {
            self
        } else {
            self.1 = vec![f(self.1)];
            self
        }
    }

    pub fn tap(self, f: impl Fn(&Self)) -> Self {
        f(&self);
        self
    }

    pub fn to_result(self) -> Result<A, Vec<E>> {
        if self.1.is_empty() {
            Ok(self.0)
        } else {
            Err(self.1)
        }
    }

    pub fn by<F, X, Y>(f: F) -> impl Fn((X, Y)) -> Result<PartialResult<A, E>, E>
    where
        F: Fn((X, Y)) -> Result<A, E>,
    {
        move |(x, y)| {
            let a = f((x, y))?;
            Ok(PartialResult(a, vec![]))
        }
    }

    pub fn optionize<F>(f: F) -> impl FnOnce(Option<A>) -> PartialResult<Option<A>, E>
    where
        F: FnOnce(A) -> Result<A, E>,
    {
        |output| output.map(f).maybe()
    }
}

impl<A, E> PartialResult<Option<A>, E> {
    pub fn ok_or_errors(self) -> Result<A, Vec<E>> {
        let PartialResult(maybe_a, errors) = self;
        maybe_a.ok_or(errors)
    }

    pub fn flat_map_if_some<B, F>(self, f: F) -> PartialResult<Option<B>, E>
    where
        F: FnOnce(A) -> PartialResult<B, E>,
    {
        let PartialResult(output, errors) = self.map(|a| a.map(f).maybe());
        output.append(errors)
    }

    pub fn try_map_if_some<B, F>(self, f: F) -> PartialResult<Option<B>, E>
    where
        F: FnOnce(A) -> Result<B, E>,
    {
        let PartialResult(output, errors) = self.map(|a| a.map(f).maybe());
        output.append(errors)
    }
}

impl<A, E> PartialResult<Vec<A>, E> {
    pub fn err(e: E) -> Self {
        PartialResult(vec![], vec![e])
    }
}

impl<A, E> PartialResult<PartialResult<A, E>, E> {
    pub fn flatten(self) -> PartialResult<A, E> {
        let PartialResult(output, mut errors1) = self;
        let PartialResult(a, mut errors2) = output;
        errors1.append(&mut errors2);
        PartialResult(a, errors1)
    }
}
