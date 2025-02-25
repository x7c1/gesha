pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    FieldNotExist { field: String },
    CannotScanYaml { detail: String },
    IncompatibleVersion { version: String },
    TypeMismatch { expected: String, found: String },
    UnknownDataType { found: String },
    Enclosed { key: String, cause: Box<Error> },
}

pub fn with_key(key: impl Into<String>) -> impl Fn(Error) -> Error {
    let key = key.into();
    move |cause| Error::Enclosed {
        key: key.clone(),
        cause: Box::new(cause),
    }
}

pub struct Output<A>(A, Vec<Error>);

impl<A> Output<A> {
    pub fn new(a: A, errors: Vec<Error>) -> Self {
        Output(a, errors)
    }
    pub fn append(mut self, errors: Vec<Error>) -> Self {
        self.1.extend(errors);
        self
    }
    pub fn to_tuple(self) -> (A, Vec<Error>) {
        (self.0, self.1)
    }
    pub fn map<B, F>(self, f: F) -> Output<B>
    where
        F: FnOnce(A) -> B,
    {
        let Self(a, errors) = self;
        let b = f(a);
        Output(b, errors)
    }
    pub fn map_errors<F>(self, f: F) -> Self
    where
        F: FnMut(Error) -> Error,
    {
        let Self(a, errors) = self;
        let errors = errors.into_iter().map(f).collect();
        Self(a, errors)
    }
}

pub trait OptionOutputOps<A> {
    fn maybe(self) -> Output<Option<A>>;
}

impl<A> OptionOutputOps<A> for Option<Output<A>> {
    fn maybe(self) -> Output<Option<A>> {
        match self {
            None => Output(None, vec![]),
            Some(Output(a, errors)) => Output(Some(a), errors),
        }
    }
}

pub trait OutputPairOps<A, B> {
    fn lift(self) -> Output<(A, B)>;
}

impl<A, B> OutputPairOps<A, B> for (A, Output<B>) {
    fn lift(self) -> Output<(A, B)> {
        let (a, Output(b, errors)) = self;
        Output((a, b), errors)
    }
}

pub trait OutputMergeOps<A> {
    fn merge(self) -> Output<Vec<A>>;
}

impl<A> OutputMergeOps<A> for Vec<Output<A>> {
    fn merge(self) -> Output<Vec<A>> {
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
