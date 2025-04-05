use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    CannotCopyFile {
        from: PathBuf,
        to: PathBuf,
        detail: String,
    },
    CannotCreateFile {
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

impl From<Error> for crate::Error {
    fn from(e: Error) -> Self {
        crate::Error::Io(e)
    }
}
