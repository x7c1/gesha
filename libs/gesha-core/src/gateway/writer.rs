use crate::Error::{CannotCopyFile, CannotCreateFile, CannotRender, CannotWriteFile, FormatFailed};
use crate::Result;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct Writer {
    pub path: PathBuf,
    pub preamble: Option<String>,
}

impl Writer {
    pub fn touch(&self) -> Result<File> {
        File::create(&self.path).map_err(|cause| CannotCreateFile {
            path: self.path.clone(),
            detail: format!("{:?}", cause),
        })
    }

    #[instrument(skip_all)]
    pub fn create_file<A: Display + Debug>(self, a: A) -> Result<()> {
        let mut file = self.touch()?;
        if let Some(preamble) = self.preamble {
            let bytes = preamble.as_bytes();
            file.write(bytes).map_err(|cause| CannotWriteFile {
                path: self.path.clone(),
                detail: format!("{:?}", cause),
            })?;
        }

        write!(file, "{}", a).map_err(|cause| CannotRender {
            path: self.path.clone(),
            detail: format!("{:?}", cause),
        })?;

        let output = format(self.path)?;
        debug!("rustfmt>\n{}", output);
        Ok(())
    }

    pub fn copy_file<A: AsRef<Path>>(self, from: A) -> Result<()> {
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
