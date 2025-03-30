use crate::Output;
use crate::core::OutputOptionOps;
use crate::error::with_key;
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
        let (title, errors_of_title) = map
            .extract::<String>("title")
            .maybe()
            .map(|maybe| maybe.unwrap_or_default())
            .into_tuple();

        let info = InfoObject { title };
        Output::ok(info).append(errors_of_title)
    }
}
