use std::path::PathBuf;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum Error {
    DiffDetected {
        output: String,
        actual: PathBuf,
        expected: PathBuf,
    },
    JoinError {
        schema_path: PathBuf,
        cause: JoinError,
    },
    TaskNotFound {
        id: String,
    },
    UnknownTestCase {
        path: String,
    },
}

impl From<Error> for crate::Error {
    fn from(e: Error) -> Self {
        crate::Error::Testing(e)
    }
}
