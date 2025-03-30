use crate::Output;
use crate::core::OutputOptionOps;
use crate::error::with_key;
use crate::v3_0::YamlExtractor;
use crate::yaml::YamlMap;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#info-object
#[derive(Debug)]
pub struct InfoObject {
    pub title: String,
}

impl InfoObject {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn from_yaml_map(map: &mut YamlMap) -> Output<InfoObject> {
        let (mut map, errors_of_info) = map
            .extract::<YamlMap>("info")
            .maybe()
            .map(|maybe| maybe.unwrap_or_default())
            .into_tuple();

        let (title, errors_of_title) = map
            .extract::<String>("title")
            .maybe()
            .map(|maybe| maybe.unwrap_or_default())
            .into_tuple();

        let info = InfoObject { title };

        Output::ok(info)
            .append(errors_of_info)
            .append(errors_of_title)
            .bind_errors(with_key("info"))
    }
}
