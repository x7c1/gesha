use crate::wire;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    Wire(wire::Error),
}

impl From<wire::Error> for Error {
    fn from(cause: wire::Error) -> Self {
        Self::Wire(cause)
    }
}
