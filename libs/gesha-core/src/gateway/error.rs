use crate::{conversions, renderer, yaml};
use std::path::PathBuf;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    // inherited errors
    Conversions(conversions::Error),
    Renderer(renderer::Error),
    Yaml(yaml::Error),

    // module errors
    FormatFailed { path: PathBuf, detail: String },
    CannotCreateFile { path: PathBuf, detail: String },
    CannotWriteFile { path: PathBuf, detail: String },
    CannotReadFile { path: PathBuf, detail: String },
    CannotRender { path: PathBuf, detail: String },
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

impl From<yaml::Error> for Error {
    fn from(cause: yaml::Error) -> Self {
        Self::Yaml(cause)
    }
}
