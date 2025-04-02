use crate::v3_0::components::ComponentsShape;
use crate::v3_0::components::schemas::{DefinitionShape, FieldShape};
use DefinitionShape::{AllOf, Enum, Mod, NewType, OneOf, Struct};
use gesha_collections::seq::{MapCollect, TryMap};
use gesha_core::broken;
use gesha_core::conversions::Result;

pub fn resolve_optionality(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shapes.schemas.root.defs;
    shapes.schemas.root.defs = defs.map_collect(resolve).to_result()?;
    Ok(shapes)
}

fn resolve(def: DefinitionShape) -> Result<DefinitionShape> {
    let def = match def {
        Struct(mut shape) => {
            shape.fields = shape.fields.try_map(transform_field)?;
            shape.into()
        }
        NewType(mut shape) => {
            shape.type_shape = shape.type_shape.resolve_optionality()?;
            shape.into()
        }
        Enum(_) => {
            // nop
            def
        }
        Mod(shape) => Mod(shape.map_def(resolve)?),
        AllOf(_) | OneOf(_) => Err(broken!(def))?,
    };
    Ok(def)
}

fn transform_field(mut shape: FieldShape) -> Result<FieldShape> {
    shape.type_shape = shape.type_shape.resolve_optionality()?;
    Ok(shape)
}
