#[derive(Debug)]
pub struct Output<A, E>(A, Vec<E>);

impl<A, E> Output<A, E> {
    pub fn new(a: A, errors: Vec<E>) -> Self {
        Output(a, errors)
    }

    pub fn append(mut self, errors: Vec<E>) -> Self {
        self.1.extend(errors);
        self
    }

    pub fn into_tuple(self) -> (A, Vec<E>) {
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

    pub fn to_result(self) -> Result<A, Vec<E>> {
        if self.1.is_empty() {
            Ok(self.0)
        } else {
            Err(self.1)
        }
    }

    pub fn by<F, X, Y>(f: F) -> impl Fn((X, Y)) -> Result<Output<A, E>, E>
    where
        F: Fn((X, Y)) -> Result<A, E>,
    {
        move |(x, y)| {
            let a = f((x, y))?;
            Ok(Output(a, vec![]))
        }
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

impl<A, E> OutputOptionOps<A, E> for Option<Result<A, E>> {
    fn maybe(self) -> Output<Option<A>, E> {
        match self {
            None => Output(None, vec![]),
            Some(Ok(a)) => Output(Some(a), vec![]),
            Some(Err(e)) => Output(None, vec![e]),
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
            let (a, mut errors) = x.into_tuple();
            xs.push(a);
            ys.append(&mut errors);
            (xs, ys)
        });
        Output(tuples.0, tuples.1)
    }
}

impl<A, E> OutputMergeOps<A, E> for Vec<Result<A, E>> {
    fn merge(self) -> Output<Vec<A>, E> {
        let init = (vec![], vec![]);
        let tuples = self.into_iter().fold(init, |(mut xs, mut ys), x| {
            match x {
                Ok(a) => xs.push(a),
                Err(e) => ys.push(e),
            }
            (xs, ys)
        });
        Output(tuples.0, tuples.1)
    }
}

impl<A, E> OutputMergeOps<A, E> for Result<Vec<A>, E> {
    fn merge(self) -> Output<Vec<A>, E> {
        match self {
            Ok(xs) => Output(xs, vec![]),
            Err(e) => Output(vec![], vec![e]),
        }
    }
}

impl<A, E> OutputMergeOps<A, E> for Output<Output<Vec<A>, E>, E> {
    fn merge(self) -> Output<Vec<A>, E> {
        let Output(xs, mut errors1) = self;
        let Output(ys, errors2) = xs;
        errors1.extend(errors2);
        Output(ys, errors1)
    }
}
