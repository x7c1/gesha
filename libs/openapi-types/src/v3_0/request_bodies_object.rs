use crate::Output;
use crate::v3_0::yaml_map_ext::collect;
use crate::v3_0::{ComponentName, RequestBodyCase};
use crate::yaml::YamlMap;
use indexmap::IndexMap;

type InnerMap = IndexMap<ComponentName, RequestBodyCase>;
type InnerEntry = (ComponentName, RequestBodyCase);

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#components-object
#[derive(Debug)]
pub struct RequestBodiesObject(InnerMap);

impl RequestBodiesObject {
    pub fn new(map: InnerMap) -> Self {
        Self(map)
    }

    pub fn from_yaml_map(map: YamlMap) -> Output<RequestBodiesObject> {
        let inner = collect(Output::by(RequestBodyCase::with_name))(map);
        inner.map(Self)
    }
}

impl FromIterator<InnerEntry> for RequestBodiesObject {
    fn from_iter<T: IntoIterator<Item = InnerEntry>>(iter: T) -> Self {
        let map = iter.into_iter().collect();
        Self::new(map)
    }
}

impl IntoIterator for RequestBodiesObject {
    type Item = InnerEntry;
    type IntoIter = <InnerMap as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
