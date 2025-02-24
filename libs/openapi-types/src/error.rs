pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    FieldNotExist { field: String },
    CannotScanYaml { detail: String },
    IncompatibleVersion { version: String },
    TypeMismatch { expected: String, found: String },
    UnknownDataType { found: String },
    Enclosed { key: String, cause: Box<Error> },
    Multiple(Vec<Error>),
}

#[derive(Debug)]
pub struct TracedError {
    pub keys: Vec<String>,
    pub cause: Error,
}

impl Error {
    pub fn with_key(key: impl Into<String>) -> impl FnOnce(Error) -> Error {
        move |cause| Error::Enclosed {
            key: key.into(),
            cause: Box::new(cause),
        }
    }
    pub fn trace_error(self) -> Vec<TracedError> {
        match self {
            Error::Enclosed { key, cause } => vec![TracedError {
                keys: vec![key],
                cause: *cause,
            }],

            Error::Multiple(errors) => errors
                .into_iter()
                .flat_map(|error| error.trace_error())
                .collect(),

            _ => vec![],
        }
    }
}
