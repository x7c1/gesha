use crate::yaml::YamlValue;
use crate::Result;

#[derive(Clone, Debug)]
pub struct YamlArray(pub(super) yaml_rust::yaml::Array);

impl IntoIterator for YamlArray {
    type Item = Result<YamlValue>;
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.0.into_iter().map(YamlValue::try_from);
        Box::new(iter)
    }
}
