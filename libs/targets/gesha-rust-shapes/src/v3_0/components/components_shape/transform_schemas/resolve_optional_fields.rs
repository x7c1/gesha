use crate::misc::TryMap;
use crate::v3_0::components::schemas::{DefinitionShape, FieldShape};
use crate::v3_0::components::ComponentsShape;
use gesha_core::broken;
use gesha_core::conversions::Result;
use DefinitionShape::{AllOf, Enum, Mod, NewType, OneOf, Struct};

pub fn resolve_optionality(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shapes.schemas.root.defs;
    let defs = defs.try_map(resolve)?;
    shapes.schemas.root.defs = defs;
    Ok(shapes)
}

fn resolve(def: DefinitionShape) -> Result<DefinitionShape> {
    let def = match def {
        Struct(mut shape) => {
            shape.fields = transform_fields(shape.fields)?;
            shape.into()
        }
        NewType { header, type_shape } => NewType {
            header,
            type_shape: type_shape.resolve_optionality()?,
        },
        Enum(_) => {
            // nop
            def
        }
        Mod(shape) => Mod(shape.map_def(resolve)?),
        AllOf(_) | OneOf(_) => Err(broken!(def))?,
    };
    Ok(def)
}

fn transform_fields(shapes: Vec<FieldShape>) -> Result<Vec<FieldShape>> {
    shapes.into_iter().map(transform_field).collect()
}

fn transform_field(mut shape: FieldShape) -> Result<FieldShape> {
    shape.type_shape = shape.type_shape.resolve_optionality()?;
    Ok(shape)
}
