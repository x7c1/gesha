pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    FieldNotExist { field: String },
    CannotScanYaml { detail: String },
    IncompatibleVersion { version: String },
    TypeMismatch { expected: String, found: String },
    UnknownDataType { found: String },
    Enclosed { key: String, causes: Vec<Error> },
    Multiple { causes: Vec<Error> },
}

impl Error {
    pub fn multiple(mut causes: Vec<Self>) -> Self {
        if causes.len() == 1 {
            causes.remove(0)
        } else {
            Self::Multiple { causes }
        }
    }
}

pub fn by_key(key: impl Into<String>) -> impl FnOnce(Error) -> Error {
    move |cause| Error::Enclosed {
        key: key.into(),
        causes: vec![cause],
    }
}

pub fn with_key(key: impl Into<String>) -> impl FnOnce(Vec<Error>) -> Error {
    move |causes| Error::Enclosed {
        key: key.into(),
        causes,
    }
}

pub type Output<A> = crate::core::Output<A, Error>;
