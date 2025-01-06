use crate::misc::TryMap;
use crate::v3_0::components::schemas::DefinitionShape::{Mod, OneOf};
use crate::v3_0::components::schemas::{
    DefinitionShape, EnumShape, EnumVariantShape, OneOfItemShape, OneOfShape,
};
use crate::v3_0::components::ComponentsShape;
use gesha_core::conversion::Error::ReferenceObjectNotFound;
use gesha_core::conversion::Result;
use gesha_rust_types::{EnumVariantName, SerdeAttribute};
use SerdeAttribute::Untagged;

pub fn convert_one_of(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        snapshot: shapes.clone(),
    };
    let defs = shapes.schemas.root.defs;
    let defs = defs.try_map(|x| transformer.shape_one_of(x))?;
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
        Ok(EnumShape {
            header: {
                shape.header.serde_attrs.push(Untagged);
                shape.header
            },
            variants: shape.items.try_map(|item| self.to_variant(item))?,
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
