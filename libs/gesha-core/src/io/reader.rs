use crate::conversions::Converter;
use crate::Error::CannotReadFile;
use crate::{Error, Output, Result};
use openapi_types::yaml::{load_from_str, ToOpenApi, YamlMap};
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use tracing::instrument;

#[derive(Debug)]
pub struct Reader {
    path: PathBuf,
}

impl Reader {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    #[instrument]
    pub fn open_target_type<A>(&self, converter: &A) -> Result<Output<A::TargetType>>
    where
        A: Converter + Debug,
    {
        let mut errors = vec![];
        let yaml = self.as_yaml_map()?;
        let (from, errors_of_openapi) = ToOpenApi::apply(yaml)
            .map_err(Error::openapi(&self.path))?
            .into_tuple();

        if !errors_of_openapi.is_empty() {
            errors.append(
                &mut errors_of_openapi
                    .into_iter()
                    .map(Error::openapi(&self.path))
                    .collect(),
            );
        };

        let (to, errors_of_conversion) = converter
            .convert(from)
            .map_err(Error::conversion(&self.path))?
            .into_tuple();

        if !errors_of_conversion.is_empty() {
            errors.append(
                &mut errors_of_conversion
                    .into_iter()
                    .map(Error::conversion(&self.path))
                    .collect(),
            );
        };

        Ok(Output::new(to, errors))
    }

    pub(crate) fn as_string(&self) -> Result<String> {
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
