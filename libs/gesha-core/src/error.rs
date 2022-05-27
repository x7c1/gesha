use crate::{conversions, wire, yaml};

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    Wire(wire::Error),
    Yaml(yaml::Error),
    Conversions(conversions::Error),
    TODO(String),
}

impl Error {
    pub fn todo<A: Into<String>>(message: A) -> Self {
        Self::TODO(message.into())
    }
}

impl From<wire::Error> for Error {
    fn from(cause: wire::Error) -> Self {
        Self::Wire(cause)
    }
}

impl From<conversions::Error> for Error {
    fn from(cause: conversions::Error) -> Self {
        Self::Conversions(cause)
    }
}

impl From<yaml::Error> for Error {
    fn from(cause: yaml::Error) -> Self {
        Self::Yaml(cause)
    }
}
