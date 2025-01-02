use console::{Style, StyledObject};
use gesha_rust_shapes::Error::TransformBroken;
use std::path::PathBuf;
use tokio::task::JoinError;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    // inherited errors
    OpenApiTypes {
        path: PathBuf,
        cause: openapi_types::Error,
    },
    Shapes {
        path: PathBuf,
        cause: gesha_rust_shapes::Error,
    },

    // thread errors
    JoinError(JoinError),

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
    Errors(Vec<Self>),
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
            Error::Shapes {
                path,
                cause: TransformBroken { detail },
            } => {
                format!(
                    "internal error: transform broken.\n{}\n{}",
                    path.display(),
                    detail,
                )
            }
            Error::Errors(errors) => errors
                .iter()
                .map(|e| e.detail(theme))
                .collect::<Vec<_>>()
                .join("\n"),

            _ => {
                format!("{:#?}", self)
            }
        }
    }
    pub fn shapes<A: Into<PathBuf>>(path: A) -> impl FnOnce(gesha_rust_shapes::Error) -> Self {
        |cause| Self::Shapes {
            path: path.into(),
            cause,
        }
    }
    pub fn openapi<A: Into<PathBuf>>(path: A) -> impl FnOnce(openapi_types::Error) -> Self {
        |cause| Self::OpenApiTypes {
            path: path.into(),
            cause,
        }
    }
    pub fn dump(&self) -> String {
        self.detail(ErrorTheme::Test)
    }
}

#[derive(Copy, Clone)]
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
