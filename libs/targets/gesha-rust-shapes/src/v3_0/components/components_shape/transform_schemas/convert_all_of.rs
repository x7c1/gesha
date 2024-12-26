use crate::v3_0::components::schemas::{DefinitionShape, StructShape};
use crate::v3_0::components::ComponentsShape;
use crate::Result;
use crate::misc::TryMap;
use DefinitionShape::{AllOf, Mod};

pub fn convert_all_of(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        snapshot: shapes.clone(),
    };
    let defs = shapes.schemas.root.defs;
    let defs = defs.try_map(|x| transformer.shape_all_of(x))?;
    shapes.schemas.root.defs = defs;
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
                    fields,
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
