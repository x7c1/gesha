use std::io;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    CannotWrite(io::Error),
}
