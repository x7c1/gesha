use crate::conversions::v3_0::to_rust_type::components::schemas::{DefinitionShape, StructShape};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;
use DefinitionShape::{AllOf, Mod};

pub fn convert_all_of(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
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
                let shape = shape.map_defs(|x| self.shape_all_of(x))?;
                Ok(Mod(shape))
            }
            _ => {
                // nop
                Ok(def)
            }
        }
    }
}
