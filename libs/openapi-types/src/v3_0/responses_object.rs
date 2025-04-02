use crate::v3_0::SpecViolation::{DuplicatedResponseSpecifier, EmptyResponses};
use crate::v3_0::yaml_extractor::collect;
use crate::v3_0::{ReferenceObject, ResponseSpecifier};
use crate::{Output, Result};
use gesha_collections::seq::VecPairs;
use gesha_collections::yaml::YamlMap;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#responses-object
#[derive(Debug)]
pub struct ResponsesObject {
    pub responses: Vec<(ResponseSpecifier, ResponseCase)>,
}

impl ResponsesObject {
    /// > The Responses Object MUST contain at least one response code,
    /// > and it SHOULD be the response for a successful operation call.
    pub fn new(responses: Vec<(ResponseSpecifier, ResponseCase)>) -> Result<Output<Self>> {
        // TODO: check if this has at least one status code
        if responses.is_empty() {
            return Err(EmptyResponses)?;
        }
        let (responses, duplicated_names) = responses.partition_unique_by_key();
        let errors = if duplicated_names.is_empty() {
            vec![]
        } else {
            let err = DuplicatedResponseSpecifier {
                fields: duplicated_names.dedup_keys(),
            };
            vec![err.into()]
        };
        Ok(Output::new(ResponsesObject { responses }, errors))
    }

    pub fn from_yaml_map(map: YamlMap) -> Result<Output<Self>> {
        collect(to_response_pair)(map)
            .map(Self::new)
            .transpose()
            .map(|output| output.flatten())
    }
}

/// Response Object | Reference Object
#[derive(Debug)]
pub enum ResponseCase {
    Response(ResponseObject),
    Reference(ReferenceObject<ResponseObject>),
}

#[derive(Debug)]
pub struct ResponseObject {}

fn to_response_pair(kv: (String, YamlMap)) -> Result<Output<(ResponseSpecifier, ResponseCase)>> {
    let (field, map) = kv;
    let specifier = ResponseSpecifier::from_string(field)?;
    let output = to_response_case(map)?.map(|case| (specifier, case));
    Ok(output)
}

fn to_response_case(_map: YamlMap) -> Result<Output<ResponseCase>> {
    // TODO:
    let case = ResponseCase::Response(ResponseObject {});
    Ok(Output::ok(case))
}
