pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    Incompatible,
    CannotParse(String),
}

impl Error {
    pub fn cannot_parse<A: Into<String>>(message: A) -> Self {
        Self::CannotParse(message.into())
    }
}
