use crate::conversions::v3_0::to_rust_type::to_shape::to_field_shapes;
use crate::conversions::v3_0::to_rust_type::to_shape::Shaper;
use crate::conversions::v3_0::to_rust_type::PostProcess::Struct;
use crate::conversions::v3_0::to_rust_type::{DefinitionShape, FieldShape};
use crate::conversions::Result;
use crate::targets::rust_type::{StructDef, StructField, TypeHeader};

impl Shaper {
    pub(super) fn for_struct(self) -> Result<DefinitionShape> {
        let field_shapes = to_field_shapes(self.object.properties, self.object.required)?;
        let fields = field_shapes
            .iter()
            .filter_map(|x| match x {
                FieldShape::Fixed(field) => Some(field.clone()),
                FieldShape::InProcess { .. } => None,
            })
            .collect::<Vec<StructField>>();

        let header = TypeHeader::new(
            self.name,
            to_doc_comments(self.object.title, self.object.description),
        );
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

fn to_doc_comments(title: Option<String>, description: Option<String>) -> Option<String> {
    let maybe = match (title, description) {
        (t, None) => t,
        (None, d) => d,
        (Some(t), Some(d)) => Some(format!("{t}\n\n{d}")),
    };
    maybe.map(|x| x.trim().to_string())
}
