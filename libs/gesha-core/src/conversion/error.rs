use openapi_types::v3_0::OpenApiDataType;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
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
    TransformBroken { detail: String },
}

#[macro_export]
macro_rules! broken {
    ($shape: expr) => {
        $crate::conversion::Error::TransformBroken {
            detail: format!(
                "unprocessed shape found:\n  at {file}:{line}\n{shape:#?}",
                file = file!(),
                line = line!(),
                shape = $shape,
            ),
        }
    };
}
