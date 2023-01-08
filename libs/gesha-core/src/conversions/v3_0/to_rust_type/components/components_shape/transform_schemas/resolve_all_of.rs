use crate::conversions::v3_0::to_rust_type::components::schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, StructShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;

pub fn resolve_all_of(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        snapshot: shapes.clone(),
    };
    let defs = shapes.schemas.root.defs;
    let defs = defs
        .into_iter()
        .map(|x| transformer.shape_all_of(x))
        .collect::<Result<Vec<_>>>()?;

    shapes.schemas.root.defs = defs;
    Ok(shapes)
}

struct Transformer {
    snapshot: ComponentsShape,
}

impl Transformer {
    fn shape_all_of(&self, def_shape: DefinitionShape) -> Result<DefinitionShape> {
        match def_shape {
            DefinitionShape::AllOf(AllOfShape { header, items }) => {
                Ok(DefinitionShape::Struct(StructShape {
                    header,
                    fields: self.merge_fields_all_of(items)?,
                }))
            }
            DefinitionShape::Mod(shape) => Ok(DefinitionShape::Mod(
                shape.map_defs(|x| self.shape_all_of(x))?,
            )),
            DefinitionShape::Struct { .. }
            | DefinitionShape::NewType { .. }
            | DefinitionShape::Enum { .. } => Ok(def_shape),
        }
    }

    fn merge_fields_all_of(&self, shapes: Vec<AllOfItemShape>) -> Result<Vec<FieldShape>> {
        let fields = shapes
            .into_iter()
            .map(|x| self.shape_item_to_fields(x))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(fields)
    }

    fn shape_item_to_fields(&self, item_shape: AllOfItemShape) -> Result<Vec<FieldShape>> {
        match item_shape {
            AllOfItemShape::Object(shapes) => Ok(shapes),
            AllOfItemShape::Ref(object) => {
                let shape = self.snapshot.find_type_definition(&object)?;
                Ok(shape.field_shapes().to_vec())
            }
        }
    }
}
