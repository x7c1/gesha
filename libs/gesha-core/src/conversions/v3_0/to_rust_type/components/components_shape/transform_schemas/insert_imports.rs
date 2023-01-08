use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, ModShape, TypeShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;
use crate::targets::rust_type::Package;
use DefinitionShape::{AllOf, Enum, Mod, NewType, Struct};

pub fn insert_imports(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    shape.schemas.root = insert_patch(shape.schemas.root, 1)?;
    Ok(shape)
}

fn insert_patch(mut shape: ModShape, depth: usize) -> Result<ModShape> {
    let is_patch_used = shape.any_type_directly(&|x| matches!(x, TypeShape::Patch(_)));
    if is_patch_used {
        shape.imports.push(Package::Patch { depth });
    }
    shape.defs = shape
        .defs
        .into_iter()
        .map(|x| match x {
            Mod(x) => Ok(insert_patch(x, depth + 1)?.into()),
            AllOf(_) | Struct(_) | NewType { .. } | Enum { .. } => Ok(x),
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(shape)
}
