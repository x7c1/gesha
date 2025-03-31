use crate::v3_0::components::ComponentsShape;
use crate::v3_0::components::schemas::{DefinitionShape, FieldShape, StructShape};
use DefinitionShape::{AllOf, Mod};
use gesha_collections::seq::MapCollect;
use gesha_core::conversions::Result;

pub fn convert_all_of(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        snapshot: shapes.clone(),
    };
    let defs = shapes.schemas.root.defs;
    shapes.schemas.root.defs = defs
        .map_collect(|x| transformer.shape_all_of(x))
        .to_result()?;

    Ok(shapes)
}

struct Transformer {
    snapshot: ComponentsShape,
}

impl Transformer {
    fn shape_all_of(&self, def: DefinitionShape) -> Result<DefinitionShape> {
        match def {
            AllOf(shape) => {
                let fields = shape.expand_fields(|x| self.snapshot.schemas.collect_fields(x));
                let next = StructShape {
                    header: shape.header,
                    fields: dedup_fields(fields),
                };
                Ok(next.into())
            }
            Mod(shape) => {
                let shape = shape.map_def(|x| self.shape_all_of(x))?;
                Ok(Mod(shape))
            }
            _ => {
                // nop
                Ok(def)
            }
        }
    }
}

fn dedup_fields(mut fields: Vec<FieldShape>) -> Vec<FieldShape> {
    let mut xs = vec![];
    while !fields.is_empty() {
        let mut x = fields.remove(0);
        (x, fields) = update_and_remove(x, fields);
        xs.push(x)
    }
    xs
}

fn update_and_remove(
    shape: FieldShape,
    mut shapes: Vec<FieldShape>,
) -> (FieldShape, Vec<FieldShape>) {
    let Some((index, _)) = shapes
        .iter()
        .enumerate()
        .find(|(_, x)| shape.name == x.name)
    else {
        return (shape, shapes);
    };
    let found = shapes.remove(index);
    let updated = shape.override_by(found);
    (updated, shapes)
}
