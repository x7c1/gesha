use crate::Output;
use crate::v3_0::yaml_extractor::collect;
use crate::v3_0::{ComponentName, SchemaCase};
use gesha_collections::yaml::YamlMap;
use indexmap::IndexMap;

/// > properties - Property definitions MUST be a Schema Object
/// > and not a standard JSON Schema (inline or referenced).
///
/// see also: https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.16
#[derive(Clone, Debug)]
pub struct SchemaProperties(IndexMap<ComponentName, SchemaCase>);

impl SchemaProperties {
    pub fn from_yaml_map(map: YamlMap) -> Output<SchemaProperties> {
        let inner = collect(Output::by(SchemaCase::with_name))(map);
        inner.map(Self)
    }
}

impl IntoIterator for SchemaProperties {
    type Item = (ComponentName, SchemaCase);
    type IntoIter = <IndexMap<ComponentName, SchemaCase> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
