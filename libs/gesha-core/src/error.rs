use crate::{conversions, yaml_wrapper};

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    Io(crate::io::Error),
    YamlWrapper(yaml_wrapper::Error),
    Conversions(conversions::Error),
    TODO(String),
}

impl Error {
    pub fn todo<A: Into<String>>(message: A) -> Self {
        Self::TODO(message.into())
    }
}

impl From<crate::io::Error> for Error {
    fn from(cause: crate::io::Error) -> Self {
        Self::Io(cause)
    }
}

impl From<conversions::Error> for Error {
    fn from(cause: conversions::Error) -> Self {
        Self::Conversions(cause)
    }
}

impl From<yaml_wrapper::Error> for Error {
    fn from(cause: yaml_wrapper::Error) -> Self {
        Self::YamlWrapper(cause)
    }
}
