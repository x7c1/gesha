use crate::conversions::Error::YamlWrapper;
use crate::yaml_wrapper;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    IncompatibleVersion,
    FieldTypeMissing,
    UnknownDataType(String),
    YamlWrapper(yaml_wrapper::Error),
}

impl From<yaml_wrapper::Error> for Error {
    fn from(cause: yaml_wrapper::Error) -> Self {
        YamlWrapper(cause)
    }
}
