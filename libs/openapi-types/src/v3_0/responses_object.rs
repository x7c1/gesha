use crate::v3_0::SpecViolation::{DuplicatedResponseSpecifier, NoResponseCode};
use crate::v3_0::yaml_extractor::collect;
use crate::v3_0::{ReferenceObject, ResponseSpecifier};
use crate::{Error, Output, Result};
use gesha_collections::seq::VecPairsOps;
use gesha_collections::yaml::YamlMap;

type Pair = (ResponseSpecifier, ResponseCase);

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#responses-object
#[derive(Debug)]
pub struct ResponsesObject {
    pub responses: Vec<Pair>,
}

impl ResponsesObject {
    /// > The Responses Object MUST contain at least one response code,
    /// > and it SHOULD be the response for a successful operation call.
    pub fn new(responses: Vec<Pair>) -> Result<Output<Self>> {
        let (responses, errors) = dedup(responses);
        ensure_status_code(&responses)?;
        Ok(Output::new(ResponsesObject { responses }, errors))
    }

    pub fn from_yaml_map(map: YamlMap) -> Result<Output<Self>> {
        collect(to_response_pair)(map)
            .map(Self::new)
            .transpose()
            .map(|output| output.flatten())
    }
}

fn dedup(responses: Vec<Pair>) -> (Vec<Pair>, Vec<Error>) {
    let (responses, duplicated_names) = responses.partition_unique_by_key();
    let errors = if duplicated_names.is_empty() {
        vec![]
    } else {
        let err = DuplicatedResponseSpecifier {
            fields: duplicated_names.dedup_keys(),
        };
        vec![err.into()]
    };
    (responses, errors)
}

fn ensure_status_code(response: &[Pair]) -> Result<()> {
    let has_status_code = response
        .iter()
        .any(|(x, _)| matches!(x, ResponseSpecifier::HttpStatusCode(_)));

    if !has_status_code {
        Err(NoResponseCode)?;
    }
    Ok(())
}

/// Response Object | Reference Object
#[derive(Debug)]
pub enum ResponseCase {
    Response(ResponseObject),
    Reference(ReferenceObject<ResponseObject>),
}

#[derive(Debug)]
pub struct ResponseObject {}

fn to_response_pair(kv: (String, YamlMap)) -> Result<Output<Pair>> {
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
