use crate::yaml_wrapper::YamlValue;

#[derive(Debug)]
pub struct YamlMap(pub(super) yaml_rust::yaml::Hash);

impl YamlMap {
    pub fn remove<A>(&mut self, key: &str) -> crate::Result<A>
    where
        A: TryFrom<YamlValue, Error = crate::Error>,
    {
        // TODO: remove unwrap
        let yaml = self.0.remove(&yaml_rust::Yaml::from_str(key)).unwrap();
        let value: YamlValue = yaml.try_into()?;
        // TODO: return error that includes (key, IncompatibleType)
        value.try_into()
    }
}
