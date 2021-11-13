use crate::errors::RequestError;
use std::collections::HashMap;

pub fn group_by_query_key(
    query_string: &str,
) -> Result<HashMap<String, Vec<String>>, RequestError> {
    let pairs: Vec<(String, String)> = serde_urlencoded::from_str(query_string)
        .map_err(|e| RequestError::QueryStringBroken(e.to_string()))?;

    let mut kvs = HashMap::<String, Vec<String>>::new();
    for (k, v) in pairs {
        kvs.entry(k).or_insert_with(Vec::new).push(v)
    }

    Ok(kvs)
}

pub fn iter_to_single_result<A, B>(xs: impl Iterator<Item = Result<A, B>>) -> Result<Vec<A>, B> {
    let mut ys = vec![];
    for x in xs {
        match x {
            Ok(value) => ys.push(value),
            Err(e) => return Err(e),
        }
    }
    Ok(ys)
}
