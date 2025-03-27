use crate::misc::MapOutput;
use crate::v3_0::components::ComponentsShape;
use crate::v3_0::components::schemas::{
    DefinitionShape, FieldShape, Optionality, TypePath, TypeShape,
};
use DefinitionShape::{AllOf, Enum, Mod, NewType, OneOf, Struct};
use gesha_core::broken;
use gesha_core::conversions::Error::Unimplemented;
use gesha_core::conversions::{Result, by_key};
use tracing::error;

pub fn resolve_type_path(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        snapshot: &shapes.clone(),
        mod_path: TypePath::new(),
    };
    let defs = shapes.schemas.root.defs;
    shapes.schemas.root.defs = defs.map_output(|x| transformer.apply(x)).to_result()?;
    Ok(shapes)
}

struct Transformer<'a> {
    snapshot: &'a ComponentsShape,
    mod_path: TypePath,
}

impl Transformer<'_> {
    fn apply(&self, def: DefinitionShape) -> Result<DefinitionShape> {
        let def = match def {
            Struct(mut shape) => {
                shape.fields = self
                    .transform_fields(shape.fields)
                    .map_err(by_key(shape.header.name.clone()))?;

                shape.into()
            }
            NewType(mut shape) => {
                shape.type_shape = self
                    .transform_field_type(shape.type_shape)
                    .map_err(by_key(shape.header.name.clone()))?;

                shape.into()
            }
            Mod(shape) => {
                let name = shape.name.clone();
                let mod_path = self.mod_path.clone().add(shape.name.clone());
                let next = shape
                    .map_def(|x| self.resolve_in_mod(mod_path.clone(), x))
                    .map_err(by_key(name))?;

                next.into()
            }
            Enum(shape) => {
                let name = shape.header.name.clone();
                let next = shape
                    .map_type(|x| self.transform_field_type(x))
                    .map_err(by_key(name))?;

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
            TypeShape::Ref(target) => {
                let is_nullable = target
                    .nullable
                    .unwrap_or_else(|| self.snapshot.schemas.is_nullable(&target));

                TypeShape::Proper {
                    data_type: self.mod_path.ancestors().add(target.type_name).into(),
                    optionality: Optionality {
                        is_required: target.is_required,
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
            TypeShape::Inline { .. } => {
                error!(
                    "unprocessed shape found: {shape:#?}\n  at {file}:{line}",
                    file = file!(),
                    line = line!()
                );
                Err(Unimplemented {
                    message: "unprocessed shape found".to_string(),
                })?
            }
        };
        Ok(resolved_type)
    }
}
