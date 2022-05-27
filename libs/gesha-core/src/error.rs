use crate::{conversions, wire};

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    Wire(wire::Error),
    Conversions(conversions::Error),
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
