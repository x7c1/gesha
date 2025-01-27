use crate::conversions::Definition;
use crate::Error::{CannotCopyFile, CannotCreateFile, CannotRender};
use crate::Result;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct Writer {
    path: PathBuf,
}

impl Writer {
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
    pub fn write_code<A: Definition>(self, a: A::TargetType) -> Result<()> {
        let mut file = self.touch()?;
        write!(file, "{}", a).map_err(|cause| CannotRender {
            path: self.path.clone(),
            detail: format!("{:?}", cause),
        })?;

        let output = A::format_code(&self.path)?;
        debug!("format>\n{}", output);
        Ok(())
    }

    pub fn copy_from<A: AsRef<Path>>(self, from: A) -> Result<()> {
        std::fs::copy(&from, &self.path).map_err(|cause| CannotCopyFile {
            from: from.as_ref().into(),
            to: self.path,
            detail: format!("{:?}", cause),
        })?;
        Ok(())
    }
}
