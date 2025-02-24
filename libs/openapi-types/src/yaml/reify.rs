use crate::yaml::YamlMap;
use crate::yaml::YamlValue;
use crate::Error::Multiple;
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
    let key: A = k.try_into()?;
    let cloned = key.to_string();
    let value = v.try_into().map_err(Error::with_key(cloned))?;
    Ok((key, value))
}

pub fn collect<X, Y, F>(f: F) -> impl FnOnce(YamlMap) -> Result<Y>
where
    F: Fn((String, YamlMap)) -> Result<X>,
    Y: FromIterator<X>,
{
    |map| {
        let (pairs, errors1) = {
            let init = (vec![], vec![]);
            fold(init, map.into_iter(), reify_entry)
        };
        let (xs, mut errors2) = {
            let init = (vec![], errors1);
            fold(init, pairs.into_iter(), f)
        };
        match errors2.len() {
            0 => Ok(xs.into_iter().collect()),
            1 => Err(errors2.remove(0)),
            _ => Err(Multiple(errors2)),
        }
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
