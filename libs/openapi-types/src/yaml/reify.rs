use crate::yaml::YamlMap;
use crate::yaml::YamlValue;
use crate::{Error, Result};
use std::fmt::Display;

pub fn reify_value<A>(v: Result<YamlValue>) -> Result<A>
where
    A: TryFrom<YamlValue, Error = Error>,
{
    v?.try_into()
}

pub fn reify_entry<A, B>(kv: Result<(YamlValue, YamlValue)>) -> Result<(A, B)>
where
    A: TryFrom<YamlValue, Error = Error> + Display,
    B: TryFrom<YamlValue, Error = Error>,
{
    let (k, v) = kv?;
    let outline = k.outline();
    let key: A = k.try_into().map_err(Error::with_key(outline))?;
    let cloned = key.to_string();
    let value = v.try_into().map_err(Error::with_key(cloned))?;
    Ok((key, value))
}

pub fn collect<X, Y, F>(f: F) -> impl FnOnce(YamlMap) -> (Y, Vec<Error>)
where
    F: Fn((String, YamlMap)) -> Result<X>,
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
        let y = xs.into_iter().collect();
        (y, errors2)
    }
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
