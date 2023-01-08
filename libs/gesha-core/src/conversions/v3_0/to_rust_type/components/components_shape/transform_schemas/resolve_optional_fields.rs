use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, FieldShape, StructShape, TypeShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;

pub fn resolve_optional_fields(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let resolver = Transformer {};
    let defs = shapes.schemas.root.defs;
    let defs = defs
        .into_iter()
        .map(|x| resolver.resolve_ref(x))
        .collect::<Result<Vec<_>>>()?;

    shapes.schemas.root.defs = defs;
    Ok(shapes)
}

struct Transformer {}

impl Transformer {
    fn resolve_ref(&self, shape: DefinitionShape) -> Result<DefinitionShape> {
        match shape {
            DefinitionShape::Struct(StructShape { header, fields }) => {
                let next = StructShape {
                    header,
                    fields: self.transform_fields(fields)?,
                };
                Ok(next.into())
            }
            DefinitionShape::NewType { header, type_shape } => {
                let next = DefinitionShape::NewType {
                    header,
                    type_shape: self.transform_field_type(type_shape)?,
                };
                Ok(next)
            }
            DefinitionShape::Enum { .. } => Ok(shape.clone()),
            DefinitionShape::AllOf { .. } => Err(PostProcessBroken {
                detail: format!(
                    "'allOf' must be processed before 'optional-fields'.\n{:#?}",
                    shape
                ),
            }),
            DefinitionShape::Mod(shape) => Ok(DefinitionShape::Mod(
                shape.map_defs(|x| self.resolve_ref(x))?,
            )),
        }
    }

    fn transform_fields(&self, shapes: Vec<FieldShape>) -> Result<Vec<FieldShape>> {
        shapes
            .into_iter()
            .map(|shape| self.transform_field(shape))
            .collect()
    }

    fn transform_field(&self, shape: FieldShape) -> Result<FieldShape> {
        Ok(FieldShape {
            name: shape.name,
            type_shape: self.transform_field_type(shape.type_shape)?,
        })
    }

    fn transform_field_type(&self, shape: TypeShape) -> Result<TypeShape> {
        let is_required = shape.is_required();
        let is_nullable = self.is_nullable(&shape)?;
        let mut expanded_type = match shape {
            TypeShape::Array { type_shape, .. } => TypeShape::Array {
                type_shape: Box::new(self.transform_field_type(*type_shape)?),
                is_required,
                is_nullable,
            },
            TypeShape::Fixed { .. } | TypeShape::Expanded { .. } => shape.clone(),
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
        match (is_required, is_nullable) {
            (true, true) | (false, false) => {
                expanded_type = TypeShape::Option(Box::new(expanded_type));
            }
            (false, true) => {
                expanded_type = TypeShape::Patch(Box::new(expanded_type));
            }
            (true, false) => {
                // nop
            }
        }
        Ok(expanded_type)
    }

    fn is_nullable(&self, shape: &TypeShape) -> Result<bool> {
        match shape {
            TypeShape::Fixed { is_nullable, .. }
            | TypeShape::Array { is_nullable, .. }
            | TypeShape::InlineObject { is_nullable, .. }
            | TypeShape::Expanded { is_nullable, .. } => Ok(*is_nullable),
            TypeShape::Ref { .. } => {
                // todo: return error
                panic!()
            }
            TypeShape::Option(..) | TypeShape::Patch(..) => todo!(),
        }
    }
}
