use super::{to_field_shapes, Shaper};
use crate::conversions::v3_0::to_rust_type::PostProcess::Struct;
use crate::conversions::v3_0::to_rust_type::{DefinitionShape, FieldShape};
use crate::conversions::Result;
use crate::targets::rust_type::{StructDef, StructField};

impl Shaper {
    pub(super) fn for_struct(self) -> Result<DefinitionShape> {
        let header = self.create_type_header();
        let field_shapes = to_field_shapes(self.object.properties, self.object.required)?;
        let fields = field_shapes
            .iter()
            .filter_map(|x| match x {
                FieldShape::Fixed(field) => Some(field.clone()),
                FieldShape::InProcess { .. } => None,
            })
            .collect::<Vec<StructField>>();

        let shape = if fields.len() == field_shapes.len() {
            let def = StructDef::new(header, fields);
            DefinitionShape::Fixed(def.into())
        } else {
            let process = Struct {
                header,
                shapes: field_shapes,
            };
            process.into()
        };
        Ok(shape)
    }
}
