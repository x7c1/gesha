use gesha_collections::partial_result::PartialResult;
use gesha_collections::tracking::{ContextAppendable, ContextBindable};
use std::fmt::Debug;

pub type Result<A> = std::result::Result<A, Error>;

pub type Output<A> = PartialResult<A, Error>;

#[derive(Debug)]
pub enum Error {
    Enclosed {
        key: String,
        cause: Box<Error>,
    },

    /// Cause: Client
    /// - `rustfmt` is missing.
    ///
    /// Cause: Internal
    /// - The generated code is malformed.
    FormatFailed {
        detail: String,
    },

    /// Cause: Client
    /// - Invalid character or unrecognized symbol in the token
    InvalidToken {
        target: String,
    },

    Multiple(Vec<Error>),

    /// Cause: Client
    /// - References a schema that does not exist.
    ReferenceObjectNotFound(String),

    /// Cause: Internal
    /// - a shape that has not been processed correctly.
    TransformBroken {
        detail: String,
    },

    /// Cause: Internal
    /// - The property is not supported.
    /// - The property is unimplemented in the current version.
    Unimplemented {
        message: String,
    },
}

impl ContextBindable<String> for Error {
    fn bind(key: impl Into<String>, causes: Vec<Self>) -> Self {
        Self::Enclosed {
            key: key.into(),
            cause: Box::new(causes.into()),
        }
    }
}

impl ContextAppendable<String> for Error {
    fn append(key: impl Into<String>, cause: Self) -> Self {
        Self::Enclosed {
            key: key.into(),
            cause: Box::new(cause),
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

#[macro_export]
macro_rules! broken {
    ($shape: expr) => {
        $crate::conversions::Error::TransformBroken {
            detail: format!(
                "unprocessed shape found:\n  at {file}:{line}\n{shape:#?}",
                file = file!(),
                line = line!(),
                shape = $shape,
            ),
        }
    };
}

#[macro_export]
macro_rules! broken_defs {
    ($name: expr) => {
        $crate::conversions::Error::TransformBroken {
            detail: format!(
                "unprocessed defs found: {name}\n  at {file}:{line}",
                name = $name,
                file = file!(),
                line = line!(),
            ),
        }
    };
}
