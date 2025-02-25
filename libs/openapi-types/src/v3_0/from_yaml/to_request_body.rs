use crate::v3_0::from_yaml::to_schema_case;
use crate::v3_0::{
    ComponentName, MediaTypeKey, MediaTypeObject, RequestBodyCase, RequestBodyObject,
};
use crate::yaml::{collect, YamlMap};
use crate::{by_key, with_key, Error, Output, Result};

pub(super) fn to_request_body_pair(
    kv: (String, YamlMap),
) -> Result<(ComponentName, RequestBodyCase)> {
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
    let (content, errors) = map
        .remove("content")
        .map(collect(Output::by(to_request_body_content_pair)))?
        .bind_errors(with_key("content"))
        .into_tuple();

    let object = RequestBodyObject {
        description: map.remove_if_exists("description")?,
        content,
        required: map.remove_if_exists("required")?.unwrap_or(false),
    };
    let output = Output::new(object, errors);
    output.to_result().map_err(Error::multiple)
}

fn to_request_body_content_pair(kv: (String, YamlMap)) -> Result<(MediaTypeKey, MediaTypeObject)> {
    let (name, map) = kv;
    let key = MediaTypeKey::new(name);
    let object = to_media_type_object(map).map_err(by_key(key.clone()))?;
    Ok((key, object))
}

fn to_media_type_object(mut map: YamlMap) -> Result<MediaTypeObject> {
    let schema = map.remove("schema").map(to_schema_case)?;
    schema.map(|schema| MediaTypeObject { schema })
}
