use crate::v3_0::components::ComponentsShape;
use crate::v3_0::components::schemas::DefinitionShape::{Enum, Mod};
use crate::v3_0::components::schemas::{EnumShape, ModShape};
use gesha_collections::seq::TryMapOps;
use gesha_core::conversions::Result;
use gesha_rust_types::{DeriveAttribute, EnumMacroForSerde, Package};

/// Transforms the shape to insert `gesha_macros::impl_enum_serde!`.
pub fn insert_macro_for_serde(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    shape.schemas.root = transform_mod(shape.schemas.root)?;
    Ok(shape)
}

fn transform_mod(mut shape: ModShape) -> Result<ModShape> {
    shape.defs = shape.defs.try_map(|x| match x {
        Mod(x) => Ok(transform_mod(x)?.into()),
        Enum(x) => Ok(transform_enum(x)?.into()),
        _ => Result::Ok(x),
    })?;

    let is_serde_derive_used = shape.any_derive_directly(|x| {
        matches!(x, DeriveAttribute::Serialize | DeriveAttribute::Deserialize)
    });
    if !is_serde_derive_used {
        shape
            .imports
            .retain(|x| !matches!(x, Package::Serialize | Package::Deserialize));
    }
    Ok(shape)
}

fn transform_enum(mut shape: EnumShape) -> Result<EnumShape> {
    let need_macro = {
        let all_string = shape.variants.iter().all(|x| x.is_string());
        let all_tuple = shape.variants.iter().all(|x| x.is_tuple());
        !all_string && !all_tuple
    };
    if !need_macro {
        return Ok(shape);
    }
    // `#[serde(rename = "...")]` is not needed
    // because `gesha_macros::impl_enum_serde` handles these mappings.
    shape
        .variants
        .iter_mut()
        .for_each(|variant| variant.erase_attributes());

    shape
        .header
        .remove_derive_attrs(&[DeriveAttribute::Serialize, DeriveAttribute::Deserialize]);

    shape.macro_for_serde = {
        let variants = shape.variants.clone().try_map(|x| x.define())?;
        let macro_impl = EnumMacroForSerde::from_variants(shape.header.name.clone(), variants);
        Some(macro_impl)
    };

    Ok(shape)
}
