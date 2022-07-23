use crate::conversions::Result;
use crate::yaml::YamlMap;
use openapi_types::v3_0::{RequestBodyCase, RequestBodyObject, SchemaFieldName};

pub(super) fn to_request_body_pair(
    kv: (String, YamlMap),
) -> Result<(SchemaFieldName, RequestBodyCase)> {
    let (name, map) = kv;
    Ok((SchemaFieldName::new(name), to_request_body_case(map)?))
}

fn to_request_body_case(mut map: YamlMap) -> Result<RequestBodyCase> {
    let case = match map.remove_if_exists::<String>("$ref")? {
        Some(_reference) => unimplemented!(),
        None => RequestBodyCase::RequestBody(Box::new(to_request_body_object(map)?)),
    };
    Ok(case)
}

fn to_request_body_object(_map: YamlMap) -> Result<RequestBodyObject> {
    Ok(RequestBodyObject {})
}
