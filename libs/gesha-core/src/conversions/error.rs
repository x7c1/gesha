use gesha_collections::partial_result::PartialResult;
use openapi_types::v3_0::OpenApiDataType;

pub type Result<A> = std::result::Result<A, Error>;

pub type Output<A> = PartialResult<A, Error>;

#[derive(Debug)]
pub enum Error {
    Enclosed {
        key: String,
        causes: Vec<Error>,
    },
    Multiple {
        causes: Vec<Error>,
    },

    /// ## Client Error
    /// e.g. a reference to a schema that does not exist.
    ReferenceObjectNotFound(String),

    /// ## Client Error
    /// e.g. a schema with an unknown format.
    UnknownFormat {
        data_type: OpenApiDataType,
        format: String,
    },

    /// ## Internal Error
    /// e.g. a shape that has not been processed.
    TransformBroken {
        detail: String,
    },

    /// ## Internal Error
    Unimplemented {
        message: String,
    },
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

impl From<Vec<Error>> for Error {
    fn from(mut causes: Vec<Error>) -> Self {
        if causes.len() == 1 {
            causes.remove(0)
        } else {
            Error::Multiple { causes }
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
