use crate::yaml::YamlLoaderError;
use crate::{json_schema, openapi, v3_0};
use gesha_collections::partial_result::PartialResult;
use gesha_collections::yaml::{KeyAppendable, KeyBindable, YamlError};
use std::fmt::Debug;

pub type Result<A> = std::result::Result<A, Error>;

pub type Output<A> = PartialResult<A, Error>;

#[derive(Debug)]
pub enum Error {
    Enclosed { key: String, causes: Vec<Error> },
    Multiple { causes: Vec<Error> },
    SpecViolation(SpecViolation),
    Unsupported(Unsupported),
    YamlLoader(YamlLoaderError),
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

impl From<Vec<Error>> for Error {
    fn from(mut causes: Vec<Error>) -> Self {
        if causes.len() == 1 {
            causes.remove(0)
        } else {
            Error::Multiple { causes }
        }
    }
}

impl From<YamlError> for Error {
    fn from(e: YamlError) -> Self {
        match e {
            YamlError::FieldNotExist { field } => {
                Error::from(openapi::SpecViolation::FieldNotExist { field })
            }
            YamlError::TypeMismatch { found, expected } => {
                Error::from(openapi::SpecViolation::TypeMismatch { found, expected })
            }
            YamlError::UnknownType { found } => {
                Error::Unsupported(Unsupported::UnknownType { found })
            }
        }
    }
}

impl KeyBindable for Error {
    fn bind_key(key: &str, error: Vec<Self>) -> Self {
        with_key(key)(error)
    }
}

impl KeyAppendable for Error {
    fn append_key(key: &str, error: Self) -> Self {
        by_key(key)(error)
    }
}

pub fn by_key(key: impl Into<String>) -> impl FnOnce(Error) -> Error {
    move |cause| Error::Enclosed {
        key: key.into(),
        causes: vec![cause],
    }
}

fn with_key(key: impl Into<String>) -> impl FnOnce(Vec<Error>) -> Error {
    move |causes| Error::Enclosed {
        key: key.into(),
        causes,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpecViolation {
    /// version independent violations
    OpenApi(openapi::SpecViolation),
    V3_0(v3_0::SpecViolation),
    JsonSchema(json_schema::SpecViolation),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Unsupported {
    IncompatibleOpenApiVersion { version: String },
    UnknownType { found: String },
    Unimplemented { message: String },
}

impl From<Unsupported> for Error {
    fn from(unsupported: Unsupported) -> Self {
        Error::Unsupported(unsupported)
    }
}
