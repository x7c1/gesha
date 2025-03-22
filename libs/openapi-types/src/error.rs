use crate::yaml::YamlLoaderError;
use crate::{json_schema, v3_0};
use std::fmt::Debug;

pub type Result<A> = std::result::Result<A, Error>;

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

#[derive(Clone, Debug, PartialEq)]
pub enum SpecViolation {
    V3_0(v3_0::SpecViolation),
    JsonSchema(json_schema::SpecViolation),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Unsupported {
    IncompatibleVersion { version: String },
    UnknownType { found: String },
}

impl From<Unsupported> for Error {
    fn from(unsupported: Unsupported) -> Self {
        Error::Unsupported(unsupported)
    }
}
