use crate::misc::TryMap;
use crate::v3_0::components::schemas::{
    DefinitionShape, FieldShape, Optionality, TypePath, TypeShape,
};
use crate::v3_0::components::ComponentsShape;
use gesha_core::broken;
use gesha_core::conversions::Result;
use DefinitionShape::{AllOf, Enum, Mod, NewType, OneOf, Struct};

pub fn resolve_type_path(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        prefix: "#/components/schemas/",
        snapshot: &shapes.clone(),
        mod_path: TypePath::new(),
    };
    let defs = shapes.schemas.root.defs;
    shapes.schemas.root.defs = defs.try_map(|x| transformer.apply(x))?;
    Ok(shapes)
}

struct Transformer<'a> {
    prefix: &'static str,
    snapshot: &'a ComponentsShape,
    mod_path: TypePath,
}

impl Transformer<'_> {
    fn apply(&self, def: DefinitionShape) -> Result<DefinitionShape> {
        let def = match def {
            Struct(mut shape) => {
                shape.fields = self.transform_fields(shape.fields)?;
                shape.into()
            }
            NewType(mut shape) => {
                shape.type_shape = self.transform_field_type(shape.type_shape)?;
                shape.into()
            }
            Mod(shape) => {
                let mod_path = self.mod_path.clone().add(shape.name.clone());
                let next = shape.map_def(|x| self.resolve_in_mod(mod_path.clone(), x))?;
                next.into()
            }
            Enum(shape) => {
                let next = shape.map_type(|x| self.transform_field_type(x))?;
                next.into()
            }
            AllOf(_) | OneOf(_) => Err(broken!(def))?,
        };
        Ok(def)
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
                target,
                is_required,
            } => {
                let is_nullable = self.snapshot.schemas.is_nullable(&target);
                let type_name = match String::from(target) {
                    x if x.starts_with(self.prefix) => x.replace(self.prefix, ""),
                    x => unimplemented!("not implemented: {x}"),
                };
                TypeShape::Proper {
                    data_type: self.mod_path.ancestors().add(type_name).into(),
                    optionality: Optionality {
                        is_required,
                        is_nullable,
                    },
                }
            }
            TypeShape::Proper { .. } => shape,
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
            TypeShape::Option(x) => TypeShape::Option(Box::new(self.transform_field_type(*x)?)),
            TypeShape::Maybe(x) => TypeShape::Maybe(Box::new(self.transform_field_type(*x)?)),
            TypeShape::Patch(x) => TypeShape::Patch(Box::new(self.transform_field_type(*x)?)),
            TypeShape::Inline { .. } => Err(broken!(shape))?,
        };
        Ok(resolved_type)
    }
}
