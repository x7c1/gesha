use super::DefinitionTransformer;
use crate::misc::MapOutput;
use crate::v3_0::components::ComponentsShape;
use crate::v3_0::components::schemas::{
    DefinitionShape, InlineSchema, InlineShape, OneOfShape, TypeShape,
};
use gesha_core::conversions::Result;

/// If `oneOf` has only one $ref,
/// replace it with a Shape containing a single `$ref`.
pub fn collapse_single_one_of(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shape.schemas.root.defs;
    let transform = <OneOfShape as DefinitionTransformer>::transform;
    shape.schemas.root.defs = defs.map_output(transform).to_result()?;
    Ok(shape)
}

impl DefinitionTransformer for OneOfShape {
    fn transform_one_of(shape: OneOfShape) -> Result<DefinitionShape> {
        if let Some(ref_shape) = shape.items.head_if_single_ref().cloned() {
            return Self::define_ref(ref_shape, shape.header);
        };
        Ok(shape.into())
    }

    fn transform_inline_one_of(schema: InlineSchema) -> Result<TypeShape> {
        if let Some(ref_shape) = schema.one_of.head_if_single_ref().cloned() {
            return Self::overwrite_ref(ref_shape, &schema.optionality);
        };
        Ok(InlineShape::OneOf(schema).into())
    }
}
