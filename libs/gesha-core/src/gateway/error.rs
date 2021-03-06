use crate::conversions::Error::PostProcessBroken;
use crate::{conversions, renderer, yaml};
use console::{Style, StyledObject};
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
    CannotCopyFile {
        from: PathBuf,
        to: PathBuf,
        detail: String,
    },
    CannotRender {
        path: PathBuf,
        detail: String,
    },
    UnsupportedExampleLocation(String),
}

impl Error {
    pub fn detail(&self, theme: ErrorTheme) -> String {
        match self {
            Error::DiffDetected {
                output,
                actual,
                expected,
            } => {
                let style = theme.diff_style();
                format!(
                    "\n {: <10} : {}\n {} : {}\n\n{}",
                    style.src_lines,
                    actual.to_string_lossy(),
                    style.dst_lines,
                    expected.to_string_lossy(),
                    output
                )
            }
            Error::FormatFailed { detail, .. } => {
                format!("rustfmt>\n{}", detail)
            }
            Error::Conversions(PostProcessBroken { detail }) => {
                format!("internal error: post-process broken.\n{}", detail)
            }
            _ => {
                format!("{:#?}", self)
            }
        }
    }
    pub fn dump(&self) {
        let message = self.detail(ErrorTheme::Test);
        println!("[failed] {}", message)
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

pub enum ErrorTheme {
    Test,
    Overwrite,
}

pub struct DiffStyle {
    src_lines: StyledObject<&'static str>,
    dst_lines: StyledObject<&'static str>,
}

impl ErrorTheme {
    pub fn diff_style(&self) -> DiffStyle {
        match self {
            ErrorTheme::Test => DiffStyle {
                src_lines: Style::new().red().apply_to("- actual"),
                dst_lines: Style::new().green().apply_to("+ expected"),
            },
            ErrorTheme::Overwrite => DiffStyle {
                src_lines: Style::new().red().apply_to("- current"),
                dst_lines: Style::new().green().apply_to("+ modified"),
            },
        }
    }
}
