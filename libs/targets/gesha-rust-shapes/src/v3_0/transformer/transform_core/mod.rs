use crate::v3_0::components::ComponentsShape;
use crate::v3_0::components::schemas::TypeShape;
use gesha_core::conversions::Result;
use gesha_rust_types::{ErrorDef, ErrorVariant, Package, PresetDef};

pub fn transform_core(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let is_patch_used = shapes.any_type(|x| matches!(x, TypeShape::Patch(_)));
    if is_patch_used {
        shapes.core.defs.set(PresetDef::Patch)?;
        shapes.core.imports.set(vec![
            Package::Deserialize,
            Package::Deserializer,
            Package::Serialize,
            Package::Serializer,
        ]);
    }
    let mut error_def = ErrorDef::new();
    if let Some(media_type) = shapes.request_bodies.define_media_type()? {
        shapes.core.imports.set(vec![
            Package::Deserialize,
            Package::Display,
            Package::Formatter,
        ]);
        shapes.core.defs.set(PresetDef::MediaType(media_type))?;
        shapes.core.defs.set(PresetDef::FromJson)?;
        error_def.set(ErrorVariant::InvalidJson);
        error_def.set(ErrorVariant::UnsupportedMediaType);
    }
    if shapes.any_enum(|shape| shape.macro_for_from.is_some()) {
        error_def.set(ErrorVariant::UnknownEnumValue);
    }
    if !error_def.is_empty() {
        shapes.core.defs.set(PresetDef::Error(error_def))?;
    }
    Ok(shapes)
}
