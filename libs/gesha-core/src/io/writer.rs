use crate::Error::{CannotCopyFile, CannotCreateFile, CannotRender, FormatFailed};
use crate::Result;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
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
    pub fn create_file<A: Display>(self, a: A) -> Result<()> {
        let mut file = self.touch()?;
        write!(file, "{}", a).map_err(|cause| CannotRender {
            path: self.path.clone(),
            detail: format!("{:?}", cause),
        })?;

        //TODO: move this formatter to conversions::Definition
        let output = format(self.path)?;
        debug!("rustfmt>\n{}", output);
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

#[instrument]
fn format(path: PathBuf) -> Result<String> {
    let output = Command::new("rustfmt")
        .arg("--verbose")
        .arg(&path)
        .output()
        .map_err(|e| FormatFailed {
            path: path.clone(),
            detail: format!("{:?}", e),
        })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(FormatFailed {
            path,
            detail: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}
