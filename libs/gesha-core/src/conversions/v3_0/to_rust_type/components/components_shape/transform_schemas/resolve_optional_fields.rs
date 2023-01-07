use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, FieldShape, ModShape, SchemasShape, StructShape, TypePath, TypeShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShapes;
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;

pub fn resolve_optional_fields(mut shapes: ComponentsShapes) -> Result<ComponentsShapes> {
    let resolver = Transformer {
        snapshot: &shapes.clone(),
    };
    let schemas = shapes
        .schemas
        .into_iter()
        .map(|x| resolver.resolve_ref(x))
        .collect::<Result<SchemasShape>>()?;

    shapes.schemas = schemas;
    Ok(shapes)
}

struct Transformer<'a> {
    snapshot: &'a ComponentsShapes,
}

impl Transformer<'_> {
    fn resolve_ref(&self, shape: DefinitionShape) -> Result<DefinitionShape> {
        match shape {
            DefinitionShape::Struct(StructShape { header, fields }) => {
                let next = StructShape {
                    header,
                    fields: self.shape_fields(fields)?,
                };
                Ok(next.into())
            }
            DefinitionShape::NewType { header, type_shape } => {
                let next = DefinitionShape::NewType {
                    header,
                    type_shape: self.shape_field_type(type_shape)?,
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
            DefinitionShape::Mod(ModShape { name, defs }) => {
                let next_defs = defs
                    .into_iter()
                    .map(|x| self.resolve_ref(x))
                    .collect::<Result<Vec<_>>>()?;

                Ok(DefinitionShape::Mod(ModShape {
                    name,
                    defs: next_defs,
                }))
            }
        }
    }

    fn shape_fields(&self, shapes: Vec<FieldShape>) -> Result<Vec<FieldShape>> {
        shapes
            .into_iter()
            .map(|shape| self.shape_field(shape))
            .collect()
    }

    fn shape_field(&self, shape: FieldShape) -> Result<FieldShape> {
        Ok(FieldShape {
            name: shape.name,
            type_shape: self.shape_field_type(shape.type_shape)?,
        })
    }

    fn shape_field_type(&self, shape: TypeShape) -> Result<TypeShape> {
        let is_required = shape.is_required();
        let is_nullable = self.is_nullable(&shape)?;
        let mut expanded_type = match shape {
            TypeShape::Array { type_shape, .. } => TypeShape::Array {
                type_shape: Box::new(self.shape_field_type(*type_shape)?),
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
