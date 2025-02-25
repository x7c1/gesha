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

pub type Output<A> = crate::core::Output<A, Error>;
