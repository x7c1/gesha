use crate::{conversions, io, testing};
use console::{Style, StyledObject};
use gesha_collections::partial_result::PartialResult;
use std::path::PathBuf;

pub type Result<A> = std::result::Result<A, Error>;

pub type Output<A> = PartialResult<A, Error>;

#[derive(Debug)]
pub enum Error {
    // inherited errors
    OpenApiTypes {
        path: PathBuf,
        cause: openapi_types::Error,
    },
    Conversion {
        path: PathBuf,
        cause: conversions::Error,
    },

    FormatFailed {
        path: PathBuf,
        detail: String,
    },

    Io(io::Error),

    Multiple(Vec<Self>),

    #[cfg(feature = "testing")]
    Testing(testing::Error),
}

impl Error {
    pub fn detail(&self, theme: ErrorTheme) -> String {
        match self {
            Error::FormatFailed { detail, .. } => {
                format!("rustfmt>\n{}", detail)
            }
            Error::Conversion {
                path,
                cause: conversions::Error::TransformBroken { detail },
            } => {
                format!(
                    "internal error: transform broken.\n{}\n{}",
                    path.display(),
                    detail,
                )
            }
            Error::Multiple(errors) => errors
                .iter()
                .map(|e| e.detail(theme))
                .collect::<Vec<_>>()
                .join("\n"),

            #[cfg(feature = "testing")]
            Error::Testing(testing::Error::DiffDetected {
                output,
                actual,
                expected,
            }) => {
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
            _ => {
                format!("{:#?}", self)
            }
        }
    }
    pub fn conversion<A: Into<PathBuf>>(path: A) -> impl Fn(conversions::Error) -> Self {
        let path = path.into();
        move |cause| Self::Conversion {
            path: path.clone(),
            cause,
        }
    }
    pub fn openapi<A: Into<PathBuf>>(path: A) -> impl Fn(openapi_types::Error) -> Self {
        let path = path.into();
        move |cause| Self::OpenApiTypes {
            path: path.clone(),
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
