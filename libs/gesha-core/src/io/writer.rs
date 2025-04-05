use crate::Result;
use crate::io::Error::{CannotCreateFile, CannotRender};
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::instrument;

#[derive(Debug)]
pub struct Writer {
    pub(crate) path: PathBuf,
}

impl Writer {
    /// path: The location where the file will be created.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn touch(&self) -> Result<File> {
        let file = File::create(&self.path).map_err(|cause| CannotCreateFile {
            path: self.path.clone(),
            detail: format!("{:?}", cause),
        })?;
        Ok(file)
    }

    #[instrument(skip_all)]
    pub fn write_code(self, code: impl Display) -> Result<()> {
        let mut file = self.touch()?;
        write!(file, "{}", code).map_err(|cause| CannotRender {
            path: self.path.clone(),
            detail: format!("{:?}", cause),
        })?;
        Ok(())
    }
}
