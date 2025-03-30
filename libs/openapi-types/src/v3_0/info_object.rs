use crate::Output;
use crate::v3_0::YamlExtractor;
use crate::yaml::YamlMap;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#info-object
#[derive(Debug)]
pub struct InfoObject {
    /// > REQUIRED. The title of the API.
    pub title: String,
}

impl InfoObject {
    pub fn from_yaml_map(mut map: YamlMap) -> Output<InfoObject> {
        let (title, errors_of_title) = map.extract_with_default("title", Output::ok).into_tuple();
        let info = InfoObject { title };
        Output::ok(info).append(errors_of_title)
    }
}
