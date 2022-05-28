use crate::conversions::Error::Yaml;
use crate::yaml;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    // inherited errors
    Yaml(yaml::Error),

    // module errors
    IncompatibleVersion,
    FieldTypeMissing,
    UnknownDataType(String),
}

impl From<yaml::Error> for Error {
    fn from(cause: yaml::Error) -> Self {
        Yaml(cause)
    }
}
