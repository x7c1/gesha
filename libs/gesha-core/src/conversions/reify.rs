use crate::conversions::Result;
use crate::yaml;
use crate::yaml::YamlMap;
use crate::yaml::YamlValue;

pub(super) fn reify_value<A>(v: yaml::Result<YamlValue>) -> Result<A>
where
    A: TryFrom<YamlValue, Error = yaml::Error>,
{
    v?.try_into().map_err(|e: yaml::Error| e.into())
}

pub(super) fn reify_entry<A, B>(kv: yaml::Result<(YamlValue, YamlValue)>) -> Result<(A, B)>
where
    A: TryFrom<YamlValue, Error = yaml::Error>,
    B: TryFrom<YamlValue, Error = yaml::Error>,
{
    match kv {
        Ok((k, v)) => Ok((k.try_into()?, v.try_into()?)),
        Err(e) => Err(e.into()),
    }
}

pub(super) fn collect<X, Y, F>(f: F) -> impl FnOnce(YamlMap) -> Result<Y>
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
