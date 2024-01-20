use crate::yaml::YamlMap;
use crate::yaml::YamlValue;
use crate::{Error, Result};

pub fn reify_value<A>(v: Result<YamlValue>) -> Result<A>
where
    A: TryFrom<YamlValue, Error = Error>,
{
    v?.try_into()
}

pub fn reify_entry<A, B>(kv: Result<(YamlValue, YamlValue)>) -> Result<(A, B)>
where
    A: TryFrom<YamlValue, Error = Error>,
    B: TryFrom<YamlValue, Error = Error>,
{
    let (k, v) = kv?;
    Ok((k.try_into()?, v.try_into()?))
}

pub fn collect<X, Y, F>(f: F) -> impl FnOnce(YamlMap) -> Result<Y>
where
    F: Fn((String, YamlMap)) -> Result<X>,
    Y: FromIterator<X>,
{
    |map| {
        map.into_iter()
            .map(reify_entry)
            .collect::<Result<Vec<(String, YamlMap)>>>()?
            .into_iter()
            .map(f)
            .collect()
    }
}
