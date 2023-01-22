use crate::conversions::v3_0::to_rust_type::components::schemas::DefinitionShape::{Mod, OneOf};
use crate::conversions::v3_0::to_rust_type::components::schemas::{DefinitionShape, EnumShape};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, EnumVariant, EnumVariantName, SerdeAttribute};

pub fn convert_one_of(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        snapshot: shapes.clone(),
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
    snapshot: ComponentsShape,
}

impl Transformer {
    #[allow(clippy::only_used_in_recursion)]
    fn shape_one_of(&self, def: DefinitionShape) -> Result<DefinitionShape> {
        match def {
            OneOf(mut shape) => {
                let variants = shape
                    .items
                    .into_iter()
                    .map(|item| {
                        let name = self
                            .snapshot
                            .schemas
                            .find_type_name(&item.target)
                            .map(EnumVariantName::new)
                            .unwrap();

                        let data_type = DataType::Custom(name.to_string());
                        EnumVariant::tuple(name, vec![data_type], vec![])
                    })
                    .collect();

                shape.header.serde_attrs.push(SerdeAttribute::Untagged);
                let next = EnumShape {
                    header: shape.header,
                    variants,
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
