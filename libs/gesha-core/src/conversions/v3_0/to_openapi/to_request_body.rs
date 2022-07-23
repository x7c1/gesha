use crate::conversions::Result;
use crate::yaml::YamlMap;
use openapi_types::v3_0::{RequestBodyCase, SchemaFieldName};

pub(super) fn to_request_body_pair(
    kv: (String, YamlMap),
) -> Result<(SchemaFieldName, RequestBodyCase)> {
    unimplemented!()
}
