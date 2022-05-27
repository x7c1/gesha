use crate::{conversions, renderer, yaml_wrapper};
use std::path::PathBuf;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    // inherited errors
    Conversions(conversions::Error),
    Renderer(renderer::Error),
    YamlWrapper(yaml_wrapper::Error),

    // module errors
    FormatFailed { path: PathBuf, detail: String },
    CannotWriteFile { path: PathBuf, detail: String },
    CannotReadFile { path: PathBuf, detail: String },
}

impl From<renderer::Error> for Error {
    fn from(cause: renderer::Error) -> Self {
        Self::Renderer(cause)
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
