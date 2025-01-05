use gesha_core::conversion::ConversionError;
use openapi_types::v3_0::OpenApiDataType;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    // client errors
    ReferenceObjectNotFound(String),

    // module errors
    TransformBroken {
        detail: String,
    },
    UnknownFormat {
        data_type: OpenApiDataType,
        format: String,
    },
}

impl From<Error> for ConversionError {
    fn from(_value: Error) -> Self {
        todo!()
    }
}

#[macro_export]
macro_rules! broken {
    ($shape: expr) => {
        $crate::Error::TransformBroken {
            detail: format!(
                "unprocessed shape found:\n  at {file}:{line}\n{shape:#?}",
                file = file!(),
                line = line!(),
                shape = $shape,
            ),
        }
    };
}
