use crate::yaml::YamlLoaderError;
use crate::{http, json_schema, openapi, v3_0};
use gesha_collections::partial_result::PartialResult;
use gesha_collections::tracking::{ContextAppendable, ContextBindable};
use gesha_collections::yaml::YamlError;
use std::fmt::Debug;

pub type Result<A> = std::result::Result<A, Error>;

pub type Output<A> = PartialResult<A, Error>;

#[derive(Debug)]
pub enum Error {
    Enclosed { key: String, causes: Vec<Error> },
    Multiple(Vec<Error>),
    SpecViolation(SpecViolation),
    Unsupported(Unsupported),
    YamlLoader(YamlLoaderError),
}

impl Error {
    pub fn multiple(mut causes: Vec<Self>) -> Self {
        if causes.len() == 1 {
            causes.remove(0)
        } else {
            Self::Multiple(causes)
        }
    }
}

impl From<Vec<Error>> for Error {
    fn from(mut causes: Vec<Error>) -> Self {
        if causes.len() == 1 {
            causes.remove(0)
        } else {
            Error::Multiple(causes)
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

impl ContextBindable<String> for Error {
    fn bind(key: impl Into<String>, causes: Vec<Self>) -> Self {
        Error::Enclosed {
            key: key.into(),
            causes,
        }
    }
}

impl ContextAppendable<String> for Error {
    fn append(key: impl Into<String>, cause: Self) -> Self {
        Error::Enclosed {
            key: key.into(),
            causes: vec![cause],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpecViolation {
    Http(http::SpecViolation),
    JsonSchema(json_schema::SpecViolation),

    /// version independent violations
    OpenApi(openapi::SpecViolation),

    V3_0(v3_0::SpecViolation),
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
