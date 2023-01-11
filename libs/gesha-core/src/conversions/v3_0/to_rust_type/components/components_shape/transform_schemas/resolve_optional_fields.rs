use crate::broken;
use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, FieldShape, StructShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;
use DefinitionShape::{AllOf, Enum, Mod, NewType, Struct};

pub fn resolve_optionality(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shapes.schemas.root.defs;
    let defs = defs.into_iter().map(resolve).collect::<Result<Vec<_>>>()?;
    shapes.schemas.root.defs = defs;
    Ok(shapes)
}

fn resolve(shape: DefinitionShape) -> Result<DefinitionShape> {
    match shape {
        Struct(StructShape { header, fields }) => {
            let next = StructShape {
                header,
                fields: transform_fields(fields)?,
            };
            Ok(next.into())
        }
        NewType { header, type_shape } => {
            let next = NewType {
                header,
                type_shape: type_shape.resolve_optionality()?,
            };
            Ok(next)
        }
        Enum { .. } => Ok(shape),
        Mod(shape) => Ok(Mod(shape.map_defs(resolve)?)),
        AllOf { .. } => Err(broken!(shape)),
    }
}

fn transform_fields(shapes: Vec<FieldShape>) -> Result<Vec<FieldShape>> {
    shapes.into_iter().map(transform_field).collect()
}

fn transform_field(mut shape: FieldShape) -> Result<FieldShape> {
    shape.type_shape = shape.type_shape.resolve_optionality()?;
    Ok(shape)
}
