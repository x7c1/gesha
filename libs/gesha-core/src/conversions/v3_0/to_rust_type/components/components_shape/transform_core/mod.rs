use crate::conversions::v3_0::to_rust_type::components::schemas::TypeShape;
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;
use crate::targets::rust_type::{ErrorDef, Package, PresetDef};

pub fn transform_core(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let error_def = ErrorDef::new();

    let is_patch_used = shapes.any_type(|x| matches!(x, TypeShape::Patch(_)));
    if is_patch_used {
        shapes.core.defs.set(PresetDef::Patch);
        shapes.core.imports.set(vec![
            Package::Deserialize,
            Package::Deserializer,
            Package::Serialize,
            Package::Serializer,
        ]);
    }

    // if let Some(media_type) = self.create_media_type_def() {
    //     imports.set(vec![
    //         Package::Deserialize,
    //         Package::Display,
    //         Package::Formatter,
    //     ]);
    //     core_defs.set(PresetDef::MediaType(media_type));
    //     core_defs.set(PresetDef::FromJson);
    //     error_def.set(ErrorVariant::InvalidJson);
    //     error_def.set(ErrorVariant::UnsupportedMediaType);
    // }

    if !error_def.is_empty() {
        shapes.core.defs.set(PresetDef::Error(error_def))
    }
    Ok(shapes)
}

// fn create_media_type_def(&self) -> Option<MediaTypeDef> {
//     let translator = self
//         .request_bodies
//         .iter()
//         .flat_map(|def| def.translate_media_types())
//         .collect::<IndexMap<EnumVariantName, &str>>();
//
//     if translator.is_empty() {
//         None
//     } else {
//         Some(MediaTypeDef { translator })
//     }
// }
