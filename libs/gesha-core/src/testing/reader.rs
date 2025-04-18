use crate::Result;
use crate::io::Reader;
use std::path::Path;

impl Reader {
    pub fn file_to_string(path: impl AsRef<Path>) -> Result<String> {
        Self::new(path.as_ref()).as_string()
    }
}
