use crate::yaml_wrapper::YamlValue;
use crate::{conversions, yaml_wrapper};

pub(super) fn reify_value<A>(v: yaml_wrapper::Result<YamlValue>) -> conversions::Result<A>
where
    A: TryFrom<YamlValue, Error = yaml_wrapper::Error>,
{
    v?.try_into().map_err(|e: yaml_wrapper::Error| e.into())
}

pub(super) fn reify_entry<A, B>(
    kv: yaml_wrapper::Result<(YamlValue, YamlValue)>,
) -> conversions::Result<(A, B)>
where
    A: TryFrom<YamlValue, Error = yaml_wrapper::Error>,
    B: TryFrom<YamlValue, Error = yaml_wrapper::Error>,
{
    match kv {
        Ok((k, v)) => Ok((k.try_into()?, v.try_into()?)),
        Err(e) => Err(e.into()),
    }
}
