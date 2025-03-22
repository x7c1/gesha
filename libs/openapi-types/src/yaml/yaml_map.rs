use crate::Error::FieldNotExist;
use crate::core::OutputOptionOps;
use crate::error::with_key;
use crate::yaml::{YamlError, YamlValue};
use crate::{Error, Output, Result};

#[derive(Clone, Debug)]
pub struct YamlMap(pub(super) yaml_rust::yaml::Hash);

impl YamlMap {
    pub fn remove<A>(&mut self, key: &str) -> Result<A>
    where
        A: TryFrom<YamlValue, Error = Error>,
    {
        self.remove_if_exists(key)?.ok_or_else(|| FieldNotExist {
            field: key.to_string(),
        })
    }

    pub fn remove_if_exists2<A>(&mut self, key: &str) -> std::result::Result<Option<A>, YamlError>
    where
        A: TryFrom<YamlValue, Error = YamlError>,
    {
        let yaml = self.0.remove(&yaml_rust::Yaml::from_str(key));
        let value: Option<YamlValue> = yaml.map(YamlValue::from_yaml).transpose()?;
        value.map(|x| x.try_into()).transpose()
    }

    pub fn remove_if_exists<A>(&mut self, key: &str) -> Result<Option<A>>
    where
        A: TryFrom<YamlValue, Error = Error>,
    {
        let yaml = self.0.remove(&yaml_rust::Yaml::from_str(key));
        let value: Option<YamlValue> = yaml.map(|x| x.try_into()).transpose()?;
        value.map(|x| x.try_into()).transpose()
    }

    pub fn extract_if_exists<A>(&mut self, key: &str) -> Output<Option<A>>
    where
        A: TryFrom<YamlValue, Error = Error>,
    {
        self.remove_if_exists::<A>(key)
            .maybe()
            .bind_errors(with_key(key))
    }

    pub fn flat_extract_if_exists<F, A, B>(&mut self, key: &str, f: F) -> Output<Option<B>>
    where
        F: FnOnce(A) -> Output<B>,
        A: TryFrom<YamlValue, Error = Error>,
    {
        self.remove_if_exists::<A>(key)
            .maybe()
            .flat_map_if_some(f)
            .bind_errors(with_key(key))
    }

    pub fn try_extract_if_exists<F, A, B>(&mut self, key: &str, f: F) -> Output<Option<B>>
    where
        F: FnOnce(A) -> Result<B>,
        A: TryFrom<YamlValue, Error = Error>,
    {
        self.remove_if_exists::<A>(key)
            .maybe()
            .try_map_if_some(f)
            .bind_errors(with_key(key))
    }
}

impl IntoIterator for YamlMap {
    type Item = Result<(YamlValue, YamlValue)>;
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.0.into_iter().map(|(k, v)| {
            match (YamlValue::try_from(k), YamlValue::try_from(v)) {
                (Ok(k), Ok(v)) => Ok((k, v)),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        });
        Box::new(iter)
    }
}
