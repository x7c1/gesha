use crate::v3_0::components::ComponentsShape;
use crate::v3_0::components::schemas::DefinitionShape::{Enum, Mod};
use crate::v3_0::components::schemas::{EnumShape, ModShape};
use gesha_collections::seq::TryMapOps;
use gesha_core::conversions::Result;
use gesha_rust_types::{EnumMacroForFrom, EnumMacroTypeForFrom, EnumMacroVariants};

/// Transform the shape to insert `gesha_macros::impl_enum_from!`.
pub fn insert_macro_for_from(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    let depth = 1;
    shape.schemas.root = transform_mod(shape.schemas.root, depth)?;
    Ok(shape)
}

fn transform_mod(mut shape: ModShape, depth: u16) -> Result<ModShape> {
    shape.defs = shape.defs.try_map(|x| match x {
        Mod(x) => Ok(transform_mod(x, depth + 1)?.into()),
        Enum(x) => Ok(transform_enum(x, depth)?.into()),
        _ => Result::Ok(x),
    })?;

    Ok(shape)
}

fn transform_enum(mut shape: EnumShape, depth: u16) -> Result<EnumShape> {
    if shape.variants.is_empty() {
        // the macro is not used for empty enums
        return Ok(shape);
    }
    let all_same_type = shape
        .variants
        .iter()
        .map(|x| x.constant().map(|y| y.type_name()))
        .collect::<Vec<_>>()
        .windows(2)
        .all(|xs| xs[0] == xs[1]);

    if !all_same_type {
        // the macro is not used for enums with different types
        return Ok(shape);
    }

    let named_constants = shape
        .variants
        .iter()
        .filter_map(|variant| variant.constant().map(|constant| (&variant.name, constant)))
        .collect::<Vec<_>>();

    let all_constant = named_constants.len() == shape.variants.len();
    if !all_constant {
        // the macro is not used for enums with non-constant variants
        return Ok(shape);
    }

    let (_name, constant) = named_constants[0];
    let Some(target_type) = EnumMacroTypeForFrom::from_constant(constant) else {
        // the macro is not used for unsupported types
        return Ok(shape);
    };

    let variants = named_constants.into_iter().fold(
        EnumMacroVariants::default(),
        |mut variants, (name, constant)| {
            variants.insert(name.clone(), constant.clone());
            variants
        },
    );

    shape.macro_for_from = Some(EnumMacroForFrom {
        name: shape.header.name.clone(),
        types: vec![target_type],
        variants,
        depth,
    });
    Ok(shape)
}
