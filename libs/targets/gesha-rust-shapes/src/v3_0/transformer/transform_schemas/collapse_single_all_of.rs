use super::DefinitionTransformer;
use crate::misc::MapOutput;
use crate::v3_0::components::ComponentsShape;
use crate::v3_0::components::schemas::{
    AllOfShape, DefinitionShape, InlineSchema, InlineShape, TypeShape,
};
use gesha_core::conversions::Result;

/// If `allOf` has only one $ref,
/// replace it with a Shape containing a single `$ref`.
pub fn collapse_single_all_of(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shape.schemas.root.defs;
    let transform = <AllOfShape as DefinitionTransformer>::transform;
    shape.schemas.root.defs = defs.map_output(transform).to_result()?;
    Ok(shape)
}

impl DefinitionTransformer for AllOfShape {
    fn transform_all_of(mut shape: AllOfShape) -> Result<DefinitionShape> {
        if let Some(ref_shape) = shape.items.head_if_single_ref().cloned() {
            return Self::define_ref(ref_shape, shape.header);
        };
        shape.items = shape.items.transform_items(Self::transform_all_of_item)?;
        Ok(shape.into())
    }

    fn transform_inline_all_of(mut schema: InlineSchema) -> Result<TypeShape> {
        if let Some(ref_shape) = schema.all_of.head_if_single_ref().cloned() {
            return Self::overwrite_ref(ref_shape, &schema.optionality);
        };
        schema.all_of = schema.all_of.transform_items(Self::transform_all_of_item)?;
        Ok(InlineShape::AllOf(schema).into())
    }
}
