pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    IncompatibleVersion,
    TypeMismatch,
    FieldTypeMissing,
    UnknownDataType(String),
    FieldNotExist { field: String },
    FormatFailed { path: String, detail: String },
    CannotReadFile { path: String, detail: String },
    CannotScanYaml { detail: String },
    CannotWriteFile { path: String, detail: String },
    TODO(String),
}

impl Error {
    pub fn todo<A: Into<String>>(message: A) -> Self {
        Self::TODO(message.into())
    }
}
