use crate::testing::CanConvert;
use crate::Error::CannotReadFile;
use crate::{Error, Result};
use openapi_types::yaml::{load_from_str, ToOpenApi, YamlMap};
use std::fs;
use std::path::{Path, PathBuf};

pub fn file_to_string<A: AsRef<Path>>(path: A) -> Result<String> {
    let content = fs::read_to_string(&path).map_err(|cause| CannotReadFile {
        path: path.as_ref().into(),
        detail: format!("{:?}", cause),
    })?;
    Ok(content)
}

pub struct Reader {
    path: PathBuf,
}

impl Reader {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn open_target_type<From, To>(&self) -> Result<To>
    where
        To: CanConvert<From>,
        From: ToOpenApi,
    {
        let yaml = self.open_yaml_map()?;
        let from: From = ToOpenApi::apply(yaml).map_err(Error::openapi(&self.path))?;
        let to: To = CanConvert::convert(from).map_err(Error::conversion(&self.path))?;
        Ok(to)
    }

    fn open_yaml_map(&self) -> Result<YamlMap> {
        let content = file_to_string(&self.path)?;
        let map = load_from_str(&content).map_err(Error::openapi(&self.path))?;
        Ok(map)
    }
}
