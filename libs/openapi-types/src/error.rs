pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    FieldNotExist { field: String },
    CannotScanYaml { detail: String },
    IncompatibleVersion { version: String },
    TypeMismatch { expected: String, found: String },
    UnknownDataType(String),
    Enclosed { key: String, cause: Box<Error> },
    Multiple(Vec<Error>),
}

impl Error {
    pub fn with_key(key: impl Into<String>) -> impl FnOnce(Error) -> Error {
        move |cause| Error::Enclosed {
            key: key.into(),
            cause: Box::new(cause),
        }
    }
}
