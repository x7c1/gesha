use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, FieldShape, Optionality, StructShape, TypePath, TypeShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;

pub fn resolve_type_path(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        prefix: "#/components/schemas/",
        snapshot: &shapes.clone(),
        mod_path: TypePath::new(),
    };
    let defs = shapes.schemas.root.defs;
    let defs = defs
        .into_iter()
        .map(|x| transformer.apply(x))
        .collect::<Result<Vec<_>>>()?;

    shapes.schemas.root.defs = defs;
    Ok(shapes)
}

struct Transformer<'a> {
    prefix: &'static str,
    snapshot: &'a ComponentsShape,
    mod_path: TypePath,
}

impl Transformer<'_> {
    fn apply(&self, shape: DefinitionShape) -> Result<DefinitionShape> {
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
            DefinitionShape::Enum { .. } => Ok(shape),
            DefinitionShape::Mod(shape) => {
                let mod_path = self.mod_path.clone().add(shape.name.clone());
                let next = shape.map_defs(|x| self.resolve_in_mod(mod_path.clone(), x))?;
                Ok(next.into())
            }
            DefinitionShape::AllOf { .. } => Err(PostProcessBroken {
                detail: format!("'allOf' must be processed before '$ref'.\n{:#?}", shape),
            }),
        }
    }

    fn resolve_in_mod(
        &self,
        mod_path: TypePath,
        shape: DefinitionShape,
    ) -> Result<DefinitionShape> {
        let resolver = Self {
            prefix: self.prefix,
            snapshot: self.snapshot,
            mod_path,
        };
        resolver.apply(shape)
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
        let resolved_type = match shape {
            TypeShape::Ref {
                object,
                is_required,
            } => {
                let is_nullable = self
                    .snapshot
                    .find_type_definition(&object)
                    .map(|def| def.is_nullable())?;

                let type_name = match String::from(object) {
                    x if x.starts_with(self.prefix) => x.replace(self.prefix, ""),
                    x => unimplemented!("not implemented: {x}"),
                };
                TypeShape::Fixed {
                    data_type: self.mod_path.ancestors().add(type_name).into(),
                    optionality: Optionality {
                        is_required,
                        is_nullable,
                    },
                }
            }
            TypeShape::Fixed { .. } => shape,
            TypeShape::Array {
                type_shape,
                optionality,
            } => TypeShape::Array {
                type_shape: Box::new(self.transform_field_type(*type_shape)?),
                optionality,
            },
            TypeShape::Expanded {
                type_path,
                optionality,
            } => TypeShape::Expanded {
                type_path: type_path.relative_from(self.mod_path.clone()),
                optionality,
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
}
