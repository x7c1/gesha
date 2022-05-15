use crate::yaml_wrapper::{YamlMap, YamlValue};
use crate::Error::IncompatibleVersion;
use crate::OpenApiDocument;
use openapi_types::v3_0::{Document, InfoObject};

mod to_paths_object;
use to_paths_object::to_paths_object;

mod to_components_object;
use to_components_object::to_components_object;

/// return Error::IncompatibleVersion if not supported version.
pub fn to_document(mut map: YamlMap) -> crate::Result<OpenApiDocument> {
    let components = map
        .remove_if_exists("components")?
        .map(to_components_object)
        .transpose()?;

    let document = Document {
        openapi: to_openapi_version(map.remove("openapi")?)?,
        info: to_info(map.remove("info")?)?,
        paths: to_paths_object(map.remove("paths")?)?,
        components,
    };
    Ok(OpenApiDocument::V3_0(document))
}

fn to_openapi_version(s: String) -> crate::Result<String> {
    if !s.starts_with("3.0.") {
        return Err(IncompatibleVersion);
    }
    Ok(s)
}

fn to_info(mut map: YamlMap) -> crate::Result<InfoObject> {
    let info = InfoObject {
        title: map.remove("title")?,
    };
    Ok(info)
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
