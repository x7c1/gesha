mod converter;
pub use converter::Converter;

mod extractor;
pub use extractor::Extractor;

mod tracking_key_appendable;
pub use tracking_key_appendable::{KeyAppendable, KeyBindable, TrackingKeyAppendable};

mod yaml_map_ext;
pub use yaml_map_ext::YamlMapExt;

use crate::yaml::{YamlError, YamlValue};

#[derive(Clone, Debug, Default)]
pub struct YamlMap(pub(super) yaml_rust::yaml::Hash);

impl YamlMap {
    pub fn remove<A>(&mut self, key: &str) -> Result<A, YamlError>
    where
        A: TryFrom<YamlValue, Error = YamlError>,
    {
        self.remove_if_exists(key)?
            .ok_or_else(|| YamlError::FieldNotExist {
                field: key.to_string(),
            })
    }

    pub fn remove_if_exists<A>(&mut self, key: &str) -> Result<Option<A>, YamlError>
    where
        A: TryFrom<YamlValue, Error = YamlError>,
    {
        let yaml = self.0.remove(&yaml_rust::Yaml::from_str(key));
        let value: Option<YamlValue> = yaml.map(YamlValue::try_from).transpose()?;
        value.map(|x| x.try_into()).transpose()
    }
}

impl IntoIterator for YamlMap {
    type Item = Result<(YamlValue, YamlValue), YamlError>;
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
