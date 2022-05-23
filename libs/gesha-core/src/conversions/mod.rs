mod v3_0;

use crate::yaml_wrapper::{YamlMap, YamlValue};

pub trait ToOpenApi: Sized {
    fn apply(map: YamlMap) -> crate::Result<Self>;
}

fn reify_value<A>(v: crate::Result<YamlValue>) -> crate::Result<A>
where
    A: TryFrom<YamlValue, Error = crate::Error>,
{
    v?.try_into()
}

fn reify_entry<A, B>(kv: crate::Result<(YamlValue, YamlValue)>) -> crate::Result<(A, B)>
where
    A: TryFrom<YamlValue, Error = crate::Error>,
    B: TryFrom<YamlValue, Error = crate::Error>,
{
    match kv {
        Ok((k, v)) => Ok((k.try_into()?, v.try_into()?)),
        Err(e) => Err(e),
    }
}
