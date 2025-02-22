use crate::Error::{CannotCopyFile, CannotCreateFile, CannotRender};
use crate::Result;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::instrument;

#[derive(Debug)]
pub struct Writer {
    path: PathBuf,
}

impl Writer {
    /// path: The location where the file will be created.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn touch(&self) -> Result<File> {
        File::create(&self.path).map_err(|cause| CannotCreateFile {
            path: self.path.clone(),
            detail: format!("{:?}", cause),
        })
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

    pub fn copy_from<B: AsRef<Path>>(self, from: B) -> Result<()> {
        std::fs::copy(&from, &self.path).map_err(|cause| CannotCopyFile {
            from: from.as_ref().into(),
            to: self.path,
            detail: format!("{:?}", cause),
        })?;
        Ok(())
    }
}
