use crate::yaml::YamlValue;
use crate::{conversions, yaml};

pub(super) fn reify_value<A>(v: yaml::Result<YamlValue>) -> conversions::Result<A>
where
    A: TryFrom<YamlValue, Error = yaml::Error>,
{
    v?.try_into().map_err(|e: yaml::Error| e.into())
}

pub(super) fn reify_entry<A, B>(
    kv: yaml::Result<(YamlValue, YamlValue)>,
) -> conversions::Result<(A, B)>
where
    A: TryFrom<YamlValue, Error = yaml::Error>,
    B: TryFrom<YamlValue, Error = yaml::Error>,
{
    match kv {
        Ok((k, v)) => Ok((k.try_into()?, v.try_into()?)),
        Err(e) => Err(e.into()),
    }
}
