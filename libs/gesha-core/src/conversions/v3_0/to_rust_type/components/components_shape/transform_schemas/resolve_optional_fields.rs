use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, FieldShape, StructShape, TypeShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;

pub fn resolve_optional_fields(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shapes.schemas.root.defs;
    let defs = defs
        .into_iter()
        .map(resolve_ref)
        .collect::<Result<Vec<_>>>()?;

    shapes.schemas.root.defs = defs;
    Ok(shapes)
}

fn resolve_ref(shape: DefinitionShape) -> Result<DefinitionShape> {
    match shape {
        DefinitionShape::Struct(StructShape { header, fields }) => {
            let next = StructShape {
                header,
                fields: transform_fields(fields)?,
            };
            Ok(next.into())
        }
        DefinitionShape::NewType { header, type_shape } => {
            let next = DefinitionShape::NewType {
                header,
                type_shape: transform_field_type(type_shape)?,
            };
            Ok(next)
        }
        DefinitionShape::Enum { .. } => Ok(shape),
        DefinitionShape::Mod(shape) => Ok(DefinitionShape::Mod(shape.map_defs(resolve_ref)?)),
        DefinitionShape::AllOf { .. } => Err(PostProcessBroken {
            detail: format!(
                "'allOf' must be processed before 'optional-fields'.\n{:#?}",
                shape
            ),
        }),
    }
}

fn transform_fields(shapes: Vec<FieldShape>) -> Result<Vec<FieldShape>> {
    shapes.into_iter().map(transform_field).collect()
}

fn transform_field(shape: FieldShape) -> Result<FieldShape> {
    Ok(FieldShape {
        name: shape.name,
        type_shape: transform_field_type(shape.type_shape)?,
    })
}

fn transform_field_type(shape: TypeShape) -> Result<TypeShape> {
    let expanded_type = match shape {
        TypeShape::Array {
            type_shape,
            optionality,
        } => {
            println!("{:#?}", optionality);
            TypeShape::Array {
                type_shape: Box::new(transform_field_type(*type_shape)?),
                optionality,
            }
        }
        TypeShape::Fixed { .. } | TypeShape::Expanded { .. } => shape,
        TypeShape::Option { .. } | TypeShape::Patch { .. } => todo!("return error"),
        TypeShape::Ref { .. } => Err(PostProcessBroken {
            detail: format!(
                "Ref must be processed before 'optional-fields'.\n{:#?}",
                shape
            ),
        })?,
        TypeShape::InlineObject { .. } => Err(PostProcessBroken {
            detail: format!(
                "InlineObject must be processed before 'optional-fields'.\n{:#?}",
                shape
            ),
        })?,
    };
    Ok(expanded_type.resolve_optionality())
}
