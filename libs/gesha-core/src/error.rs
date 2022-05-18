pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    IncompatibleVersion,
    TypeMismatch,
    FieldTypeMissing,
    TODO(String),
}

impl Error {
    pub fn todo<A: Into<String>>(message: A) -> Self {
        Self::TODO(message.into())
    }
}
