use crate::misc::MapOutput;
use crate::v3_0::components::components_shape::transform_schemas::definition_transformer::DefinitionTransformer;
use crate::v3_0::components::schemas::{
    AllOfShape, DefinitionShape, InlineSchema, InlineShape, NewTypeShape, TypeShape,
};
use crate::v3_0::components::ComponentsShape;
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
        if let Some(mut ref_shape) = shape.items.head_if_single_ref().cloned() {
            ref_shape.nullable = Some(shape.header.is_nullable);

            let type_shape = TypeShape::from(ref_shape);
            let def_shape = NewTypeShape::new(shape.header, type_shape);
            return Ok(def_shape.into());
        };
        shape.items = shape.items.transform_items(Self::transform_all_of_item)?;
        Ok(shape.into())
    }

    fn transform_inline_all_of(mut schema: InlineSchema) -> Result<TypeShape> {
        if let Some(mut ref_shape) = schema.all_of.head_if_single_ref().cloned() {
            ref_shape.is_required = schema.optionality.is_required;
            ref_shape.nullable = Some(schema.optionality.is_nullable);
            return Ok(TypeShape::Ref(ref_shape));
        };
        schema.all_of = schema.all_of.transform_items(Self::transform_all_of_item)?;
        Ok(InlineShape::AllOf(schema).into())
    }
}
