use crate::core::OutputPairOps;
use crate::v3_0::from_yaml::to_schema_case;
use crate::v3_0::{
    ComponentName, MediaTypeKey, MediaTypeObject, RequestBodyCase, RequestBodyObject,
};
use crate::yaml::{collect, YamlMap};
use crate::{Output, Result};

pub(super) fn to_request_body_pair(
    kv: (String, YamlMap),
) -> Result<Output<(ComponentName, RequestBodyCase)>> {
    let (name, map) = kv;
    let pair = (ComponentName::new(name), to_request_body_case(map)?);
    Ok(pair.lift())
}

fn to_request_body_case(mut map: YamlMap) -> Result<Output<RequestBodyCase>> {
    let case = match map.remove_if_exists::<String>("$ref")? {
        Some(_reference) => unimplemented!(),
        None => {
            let output = to_request_body_object(map)?;
            output.map(|x| RequestBodyCase::RequestBody(Box::new(x)))
        }
    };
    Ok(case)
}

fn to_request_body_object(mut map: YamlMap) -> Result<Output<RequestBodyObject>> {
    let (content, errors) = map
        .remove("content")
        .map(collect(to_request_body_content_pair))?
        .to_tuple();

    let object = RequestBodyObject {
        description: map.remove_if_exists("description")?,
        content,
        required: map.remove_if_exists("required")?.unwrap_or(false),
    };
    Ok(Output::new(object, errors))
}

fn to_request_body_content_pair(
    kv: (String, YamlMap),
) -> Result<Output<(MediaTypeKey, MediaTypeObject)>> {
    let (name, map) = kv;
    let key = MediaTypeKey::new(name);
    let output = to_media_type_object(map)?.map(|object| (key, object));
    Ok(output)
}

fn to_media_type_object(mut map: YamlMap) -> Result<Output<MediaTypeObject>> {
    let schema = map.remove("schema").map(to_schema_case)??;
    let output = schema.map(|schema| MediaTypeObject { schema });
    Ok(output)
}
