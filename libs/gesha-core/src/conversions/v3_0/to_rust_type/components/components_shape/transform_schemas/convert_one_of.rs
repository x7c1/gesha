use crate::conversions::v3_0::to_rust_type::components::schemas::DefinitionShape::{Mod, OneOf};
use crate::conversions::v3_0::to_rust_type::components::schemas::{DefinitionShape, EnumShape};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;

pub fn convert_one_of(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        _snapshot: shapes.clone(),
    };
    let defs = shapes.schemas.root.defs;
    let defs = defs
        .into_iter()
        .map(|x| transformer.shape_one_of(x))
        .collect::<Result<Vec<_>>>()?;

    shapes.schemas.root.defs = defs;
    Ok(shapes)
}

struct Transformer {
    _snapshot: ComponentsShape,
}

impl Transformer {
    #[allow(clippy::only_used_in_recursion)]
    fn shape_one_of(&self, def: DefinitionShape) -> Result<DefinitionShape> {
        match def {
            OneOf(shape) => {
                let next = EnumShape {
                    header: shape.header,
                    // TODO:
                    variants: vec![],
                };
                Ok(next.into())
            }
            Mod(shape) => {
                let shape = shape.map_defs(|x| self.shape_one_of(x))?;
                Ok(Mod(shape))
            }
            _ => {
                // nop
                Ok(def)
            }
        }
    }
}
