use crate::Output;
use crate::v3_0::{
    ComponentName, MediaTypeKey, MediaTypeObject, RequestBodyCase, RequestBodyObject, SchemaCase,
    YamlExtractor,
};
use crate::yaml::{YamlMap, collect};
use crate::{Error, Result, by_key, with_key};
use indexmap::IndexMap;

type InnerMap = IndexMap<ComponentName, RequestBodyCase>;
type InnerEntry = (ComponentName, RequestBodyCase);

/// rf. https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsObject
#[derive(Debug)]
pub struct RequestBodiesObject(InnerMap);

impl RequestBodiesObject {
    pub fn new(map: InnerMap) -> Self {
        Self(map)
    }

    pub fn from_yaml_map(map: YamlMap) -> Output<RequestBodiesObject> {
        let inner = collect(Output::by(to_request_body_pair))(map);
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

fn to_request_body_pair(kv: (String, YamlMap)) -> Result<(ComponentName, RequestBodyCase)> {
    let (name, map) = kv;
    let pair = (ComponentName::new(name), to_request_body_case(map)?);
    Ok(pair)
}

fn to_request_body_case(mut map: YamlMap) -> Result<RequestBodyCase> {
    let case = match map.remove_if_exists::<String>("$ref")? {
        Some(_reference) => unimplemented!(),
        None => {
            let object = to_request_body_object(map)?;
            RequestBodyCase::RequestBody(Box::new(object))
        }
    };
    Ok(case)
}

fn to_request_body_object(mut map: YamlMap) -> Result<RequestBodyObject> {
    let (content, errors_of_content) = map
        .remove("content")
        .map(collect(Output::by(to_request_body_content_pair)))?
        .bind_errors(with_key("content"))
        .into_tuple();

    let (required, errors_of_required) = map
        .extract_if_exists2("required")
        .map(|maybe| maybe.unwrap_or(false))
        .into_tuple();

    let object = RequestBodyObject {
        description: map.remove_if_exists("description")?,
        content,
        required,
    };
    let output = Output::ok(object)
        .append(errors_of_content)
        .append(errors_of_required);

    output.to_result().map_err(Error::multiple)
}

fn to_request_body_content_pair(kv: (String, YamlMap)) -> Result<(MediaTypeKey, MediaTypeObject)> {
    let (name, map) = kv;
    let key = MediaTypeKey::new(name);
    let object = to_media_type_object(map).map_err(by_key(key.clone()))?;
    Ok((key, object))
}

fn to_media_type_object(mut map: YamlMap) -> Result<MediaTypeObject> {
    let schema = map.remove("schema").map(SchemaCase::from_yaml_map)?;
    schema.map(|schema| MediaTypeObject { schema })
}
