use crate::Result;
use crate::io::Error::CannotCopyFile;
use crate::io::Writer;
use std::path::Path;

impl Writer {
    pub fn copy_from<B: AsRef<Path>>(self, from: B) -> Result<()> {
        std::fs::copy(&from, &self.path).map_err(|cause| CannotCopyFile {
            from: from.as_ref().into(),
            to: self.path,
            detail: format!("{:?}", cause),
        })?;
        Ok(())
    }
}
