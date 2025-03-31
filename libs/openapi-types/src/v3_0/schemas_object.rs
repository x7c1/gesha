use crate::Output;
use crate::v3_0::yaml_map_ext::collect;
use crate::v3_0::{ComponentName, SchemaCase};
use gesha_collections::yaml::YamlMap;
use indexmap::IndexMap;

#[derive(Clone, Debug)]
pub struct SchemasObject(IndexMap<ComponentName, SchemaCase>);

impl SchemasObject {
    pub fn from_yaml_map(map: YamlMap) -> Output<SchemasObject> {
        let inner = collect(Output::by(SchemaCase::with_name))(map);
        inner.map(Self)
    }
}

impl IntoIterator for SchemasObject {
    type Item = (ComponentName, SchemaCase);
    type IntoIter = <IndexMap<ComponentName, SchemaCase> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
