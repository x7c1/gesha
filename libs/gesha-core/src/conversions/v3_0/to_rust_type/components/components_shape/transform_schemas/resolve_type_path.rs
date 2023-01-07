use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, FieldShape, ModShape, SchemasShape, StructShape, TypePath, TypeShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShapes;
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;

pub fn resolve_type_path(mut shapes: ComponentsShapes) -> Result<ComponentsShapes> {
    let transformer = Transformer {
        prefix: "#/components/schemas/",
        snapshot: &shapes.clone(),
        mod_path: TypePath::new(),
    };
    let schemas = shapes
        .schemas
        .into_iter()
        .map(|x| transformer.resolve_ref(x))
        .collect::<Result<SchemasShape>>()?;

    shapes.schemas = schemas;
    Ok(shapes)
}

struct Transformer<'a> {
    prefix: &'static str,
    snapshot: &'a ComponentsShapes,
    mod_path: TypePath,
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
                detail: format!("'allOf' must be processed before '$ref'.\n{:#?}", shape),
            }),
            DefinitionShape::Mod(ModShape { name, defs }) => {
                let mod_path = self.mod_path.clone().add(name.clone());
                let next_defs = defs
                    .into_iter()
                    .map(|x| self.resolve_ref_in_mod(mod_path.clone(), x))
                    .collect::<Result<Vec<_>>>()?;

                Ok(DefinitionShape::Mod(ModShape {
                    name,
                    defs: next_defs,
                }))
            }
        }
    }

    fn resolve_ref_in_mod(
        &self,
        mod_path: TypePath,
        shape: DefinitionShape,
    ) -> Result<DefinitionShape> {
        let resolver = Self {
            prefix: self.prefix,
            snapshot: self.snapshot,
            mod_path,
        };
        resolver.resolve_ref(shape)
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
        let resolved_type = match shape {
            TypeShape::Array { type_shape, .. } => TypeShape::Array {
                type_shape: Box::new(self.shape_field_type(*type_shape)?),
                is_required,
                is_nullable,
            },
            TypeShape::Ref { object, .. } => {
                let type_name = match String::from(object) {
                    x if x.starts_with(self.prefix) => x.replace(self.prefix, ""),
                    x => unimplemented!("not implemented: {x}"),
                };
                TypeShape::Fixed {
                    data_type: self.mod_path.ancestors().add(type_name).into(),
                    is_required,
                    is_nullable,
                }
            }
            TypeShape::Fixed { .. } => shape.clone(),
            TypeShape::Expanded { type_path, .. } => TypeShape::Expanded {
                type_path: type_path.relative_from(self.mod_path.clone()),
                is_required,
                is_nullable,
            },
            TypeShape::Option(_) | TypeShape::Patch(_) => todo!("return error"),
            TypeShape::InlineObject { .. } => Err(PostProcessBroken {
                detail: format!(
                    "InlineObject must be processed before '$ref'.\n{:#?}",
                    shape
                ),
            })?,
        };
        Ok(resolved_type)
    }

    fn is_nullable(&self, shape: &TypeShape) -> Result<bool> {
        match shape {
            TypeShape::Fixed { is_nullable, .. }
            | TypeShape::Array { is_nullable, .. }
            | TypeShape::InlineObject { is_nullable, .. }
            | TypeShape::Expanded { is_nullable, .. } => Ok(*is_nullable),
            TypeShape::Ref { object, .. } => self
                .snapshot
                .find_type_definition(object)
                .map(|def| def.is_nullable()),
            TypeShape::Option(_) | TypeShape::Patch(_) => todo!(),
        }
    }
}
