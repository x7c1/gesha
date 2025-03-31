use crate::{Error, Output, Result, by_key};
use gesha_collections::partial_result::MergeOps;
use gesha_collections::yaml::{YamlError, YamlMap, YamlValue};
use std::fmt::Display;

pub fn reify_value<A>(v: std::result::Result<YamlValue, YamlError>) -> Result<A>
where
    A: TryFrom<YamlValue, Error = YamlError>,
{
    v.map_err(Error::from)?.try_into().map_err(Error::from)
}

pub fn collect<X, Y, F>(f: F) -> impl FnOnce(YamlMap) -> Output<Y>
where
    F: Fn((String, YamlMap)) -> Result<Output<X>>,
    Y: FromIterator<X>,
{
    |map| {
        let (pairs, errors1) = {
            let init = (vec![], vec![]);
            fold(init, map.into_iter(), reify_entry)
        };
        let (xs, errors2) = {
            let init = (vec![], errors1);
            fold(init, pairs.into_iter(), f)
        };
        let output = xs.merge().append(errors2);
        output.map(|xs| xs.into_iter().collect())
    }
}

fn reify_entry<A, B>(kv: std::result::Result<(YamlValue, YamlValue), YamlError>) -> Result<(A, B)>
where
    A: TryFrom<YamlValue, Error = YamlError> + Display,
    B: TryFrom<YamlValue, Error = YamlError>,
{
    let (k, v) = kv.map_err(Error::from)?;
    let outline = k.outline();
    let key: A = k.try_into().map_err(Error::from).map_err(by_key(outline))?;

    let cloned = key.to_string();
    let value = v.try_into().map_err(Error::from).map_err(by_key(cloned))?;

    Ok((key, value))
}

fn fold<X, Y, T, F>(init: (Vec<Y>, Vec<Error>), iter: T, f: F) -> (Vec<Y>, Vec<Error>)
where
    T: Iterator<Item = X>,
    F: Fn(X) -> Result<Y>,
{
    iter.map(f).fold(init, |(mut xs, mut errors), y| {
        match y {
            Ok(x) => xs.push(x),
            Err(e) => errors.push(e),
        }
        (xs, errors)
    })
}
