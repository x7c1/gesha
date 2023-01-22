use crate::conversions::v3_0::to_rust_type::components::schemas::DefinitionShape::{Mod, OneOf};
use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, EnumShape, EnumVariantShape, OneOfItemShape, OneOfShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Error::ReferenceObjectNotFound;
use crate::conversions::Result;
use crate::targets::rust_type::{EnumVariantName, SerdeAttribute};
use SerdeAttribute::Untagged;

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
    fn shape_one_of(&self, def: DefinitionShape) -> Result<DefinitionShape> {
        match def {
            OneOf(shape) => {
                let shape = self.convert_to_enum(shape)?;
                Ok(shape.into())
            }
            Mod(shape) => {
                let shape = shape.map_def(|x| self.shape_one_of(x))?;
                Ok(Mod(shape))
            }
            _ => {
                // nop
                Ok(def)
            }
        }
    }

    fn convert_to_enum(&self, mut shape: OneOfShape) -> Result<EnumShape> {
        let variants = shape
            .items
            .into_iter()
            .map(|item| self.to_variant(item))
            .collect::<Result<Vec<_>>>()?;

        Ok(EnumShape {
            header: {
                shape.header.serde_attrs.push(Untagged);
                shape.header
            },
            variants,
        })
    }

    fn to_variant(&self, item: OneOfItemShape) -> Result<EnumVariantShape> {
        let name = self
            .snapshot
            .schemas
            .find_type_name(&item.target)
            .ok_or_else(|| ReferenceObjectNotFound(item.target.clone().into()))
            .map(EnumVariantName::new)?;

        Ok(EnumVariantShape::tuple(name, vec![item.target], vec![]))
    }
}
