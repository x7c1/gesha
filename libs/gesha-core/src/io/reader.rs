use crate::testing::CanConvert;
use crate::Error::CannotReadFile;
use crate::{Error, Result};
use openapi_types::yaml::{load_from_str, ToOpenApi, YamlMap};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Reader {
    path: PathBuf,
}

impl Reader {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn file_to_string(path: impl AsRef<Path>) -> Result<String> {
        Self::new(path.as_ref()).as_string()
    }

    pub fn open_target_type<From, To>(&self) -> Result<To>
    where
        To: CanConvert<From>,
        From: ToOpenApi,
    {
        let yaml = self.as_yaml_map()?;
        let from: From = ToOpenApi::apply(yaml).map_err(Error::openapi(&self.path))?;
        let to: To = CanConvert::convert(from).map_err(Error::conversion(&self.path))?;
        Ok(to)
    }

    fn as_string(&self) -> Result<String> {
        let content = fs::read_to_string(&self.path).map_err(|cause| CannotReadFile {
            path: PathBuf::from(&self.path),
            detail: format!("{:?}", cause),
        })?;
        Ok(content)
    }

    fn as_yaml_map(&self) -> Result<YamlMap> {
        let content = self.as_string()?;
        let map = load_from_str(&content).map_err(Error::openapi(&self.path))?;
        Ok(map)
    }
}
