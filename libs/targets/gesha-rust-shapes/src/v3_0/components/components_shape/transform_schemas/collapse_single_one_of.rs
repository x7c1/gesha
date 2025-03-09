use crate::misc::MapOutput;
use crate::v3_0::components::components_shape::transform_schemas::definition_transformer::DefinitionTransformer;
use crate::v3_0::components::schemas::{
    DefinitionShape, InlineSchema, InlineShape, NewTypeShape, OneOfShape, TypeShape,
};
use crate::v3_0::components::ComponentsShape;
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
        if let Some(mut ref_shape) = shape.items.head_if_single_ref().cloned() {
            ref_shape.nullable = Some(shape.header.is_nullable);

            let type_shape = TypeShape::from(ref_shape);
            let def_shape = NewTypeShape::new(shape.header, type_shape);
            return Ok(def_shape.into());
        };
        Ok(shape.into())
    }

    fn transform_inline_one_of(schema: InlineSchema) -> Result<TypeShape> {
        if let Some(mut ref_shape) = schema.one_of.head_if_single_ref().cloned() {
            ref_shape.is_required = schema.optionality.is_required;
            ref_shape.nullable = Some(schema.optionality.is_nullable);
            return Ok(TypeShape::Ref(ref_shape));
        };
        Ok(InlineShape::OneOf(schema).into())
    }
}
