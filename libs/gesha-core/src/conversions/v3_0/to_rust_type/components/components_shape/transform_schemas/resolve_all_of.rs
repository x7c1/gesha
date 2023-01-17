use crate::conversions::v3_0::to_rust_type::components::schemas::{DefinitionShape, StructShape};
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
    fn shape_all_of(&self, def: DefinitionShape) -> Result<DefinitionShape> {
        match def {
            DefinitionShape::AllOf(shape) => {
                let fields = shape.expand_fields(|x| self.snapshot.schemas.collect_fields(x));
                let next = StructShape {
                    header: shape.header,
                    fields,
                };
                Ok(next.into())
            }
            DefinitionShape::Mod(shape) => Ok(DefinitionShape::Mod(
                shape.map_defs(|x| self.shape_all_of(x))?,
            )),
            _ => Ok(def),
        }
    }
}
