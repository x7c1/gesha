use crate::conversions::Error::Yaml;
use crate::yaml;
use openapi_types::v3_0::OpenApiDataType;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    // inherited errors
    Yaml(yaml::Error),

    // module errors
    IncompatibleVersion,
    PostProcessBroken {
        detail: String,
    },
    UnknownFormat {
        data_type: OpenApiDataType,
        format: String,
    },
    UnknownDataType(String),
}

impl From<yaml::Error> for Error {
    fn from(cause: yaml::Error) -> Self {
        Yaml(cause)
    }
}
