use crate::yaml_wrapper::YamlValue;

#[derive(Debug)]
pub struct YamlMap(pub(super) yaml_rust::yaml::Hash);

impl YamlMap {
    pub fn remove<A>(&mut self, key: &str) -> crate::Result<A>
    where
        A: TryFrom<YamlValue, Error = crate::Error>,
    {
        let maybe = self.remove_if_exists(key)?;
        // TODO: remove unwrap
        // TODO: return error that includes (key, NotFoundError)
        let x = maybe.unwrap();
        Ok(x)
    }

    pub fn remove_if_exists<A>(&mut self, key: &str) -> crate::Result<Option<A>>
    where
        A: TryFrom<YamlValue, Error = crate::Error>,
    {
        let yaml = self.0.remove(&yaml_rust::Yaml::from_str(key));
        let value: Option<YamlValue> = yaml.map(|x| x.try_into()).transpose()?;
        // TODO: return error that includes (key, IncompatibleType)
        value.map(|x| x.try_into()).transpose()
    }
}

impl IntoIterator for YamlMap {
    type Item = crate::Result<(YamlValue, YamlValue)>;
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
