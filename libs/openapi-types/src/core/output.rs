pub struct Output<A, E>(A, Vec<E>);

impl<A, E> Output<A, E> {
    pub fn new(a: A, errors: Vec<E>) -> Self {
        Output(a, errors)
    }
    pub fn append(mut self, errors: Vec<E>) -> Self {
        self.1.extend(errors);
        self
    }
    pub fn to_tuple(self) -> (A, Vec<E>) {
        (self.0, self.1)
    }
    pub fn map<B, F>(self, f: F) -> Output<B, E>
    where
        F: FnOnce(A) -> B,
    {
        let Self(a, errors) = self;
        let b = f(a);
        Output(b, errors)
    }
    pub fn map_errors<F>(self, f: F) -> Self
    where
        F: FnMut(E) -> E,
    {
        let Self(a, errors) = self;
        let errors = errors.into_iter().map(f).collect();
        Self(a, errors)
    }
}

pub trait OutputOptionOps<A, E> {
    fn maybe(self) -> Output<Option<A>, E>;
}

impl<A, E> OutputOptionOps<A, E> for Option<Output<A, E>> {
    fn maybe(self) -> Output<Option<A>, E> {
        match self {
            None => Output(None, vec![]),
            Some(Output(a, errors)) => Output(Some(a), errors),
        }
    }
}

pub trait OutputPairOps<A, B, E> {
    fn lift(self) -> Output<(A, B), E>;
}

impl<A, B, E> OutputPairOps<A, B, E> for (A, Output<B, E>) {
    fn lift(self) -> Output<(A, B), E> {
        let (a, Output(b, errors)) = self;
        Output((a, b), errors)
    }
}

pub trait OutputMergeOps<A, E> {
    fn merge(self) -> Output<Vec<A>, E>;
}

impl<A, E> OutputMergeOps<A, E> for Vec<Output<A, E>> {
    fn merge(self) -> Output<Vec<A>, E> {
        let init = (vec![], vec![]);
        let tuples = self.into_iter().fold(init, |(mut xs, mut ys), x| {
            let (a, mut errors) = x.to_tuple();
            xs.push(a);
            ys.append(&mut errors);
            (xs, ys)
        });
        Output(tuples.0, tuples.1)
    }
}
