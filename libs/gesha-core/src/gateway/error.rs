use crate::{conversions, renderer, yaml};
use console::Style;
use std::path::PathBuf;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    // inherited errors
    Conversions(conversions::Error),
    Renderer(renderer::Error),
    Yaml(yaml::Error),

    // module errors
    DiffDetected {
        output: String,
        actual: PathBuf,
        expected: PathBuf,
    },
    FormatFailed {
        path: PathBuf,
        detail: String,
    },
    CannotCreateFile {
        path: PathBuf,
        detail: String,
    },
    CannotWriteFile {
        path: PathBuf,
        detail: String,
    },
    CannotReadFile {
        path: PathBuf,
        detail: String,
    },
    CannotRender {
        path: PathBuf,
        detail: String,
    },
}

impl Error {
    pub fn dump(&self) {
        match self {
            Error::DiffDetected {
                output,
                actual,
                expected,
            } => {
                println!(
                    "[failed]\n {}   : {}\n {} : {}\n\n{}",
                    Style::new().red().apply_to("- actual"),
                    actual.to_string_lossy(),
                    Style::new().green().apply_to("+ expected"),
                    expected.to_string_lossy(),
                    output
                )
            }
            _ => {
                println!("[failed] {:#?}", self)
            }
        }
    }
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
