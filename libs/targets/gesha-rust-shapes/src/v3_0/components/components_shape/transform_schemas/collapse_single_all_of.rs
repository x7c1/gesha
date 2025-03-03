use crate::misc::{MapOutput, TryMap};
use crate::v3_0::components::schemas::DefinitionShape::{AllOf, Mod};
use crate::v3_0::components::schemas::{
    AllOfShape, DefinitionShape, FieldShape, InlineShape, NewTypeShape, StructShape, TypeShape,
};
use crate::v3_0::components::ComponentsShape;
use gesha_core::broken;
use gesha_core::conversions::Result;
use DefinitionShape::{Enum, NewType, OneOf, Struct};

/// If `allOf` has only one $ref,
/// replace it with a Schema Object containing a single `$ref`.
pub fn collapse_single_all_of(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shape.schemas.root.defs;
    shape.schemas.root.defs = defs.map_output(transform).to_result()?;
    Ok(shape)
}

fn transform(def: DefinitionShape) -> Result<DefinitionShape> {
    let transformed = match def {
        Struct(shape) => transform_struct(shape)?.into(),
        AllOf(shape) => transform_all_of(shape)?,
        NewType(shape) => transform_new_type(shape)?.into(),
        Enum(_) => {
            // enum has no shape to transform
            def
        }
        OneOf(_) => {
            // TODO:
            def
        }
        Mod(_) => return Err(broken!(def)),
    };
    Ok(transformed)
}

fn transform_struct(mut shape: StructShape) -> Result<StructShape> {
    shape.fields = shape.fields.try_map(transform_field)?;
    Ok(shape)
}

/// return NewTypeShape if given AllOfShape has only one $ref
fn transform_all_of(shape: AllOfShape) -> Result<DefinitionShape> {
    let Some(ref_shape) = shape.pop_if_only_one_ref() else {
        // TODO: convert nested items as well
        return Ok(shape.into());
    };
    let type_shape = TypeShape::from(ref_shape);
    let def_shape = NewTypeShape::new(shape.header, type_shape);
    Ok(def_shape.into())
}

fn transform_new_type(shape: NewTypeShape) -> Result<NewTypeShape> {
    Ok(shape)
}

fn transform_field(mut field: FieldShape) -> Result<FieldShape> {
    field.type_shape = transform_type_shape(field.type_shape)?;
    Ok(field)
}

fn transform_type_shape(shape: TypeShape) -> Result<TypeShape> {
    match shape {
        TypeShape::Inline(shape) => transform_inline_shape(shape),
        TypeShape::Proper { .. }
        | TypeShape::Array { .. }
        | TypeShape::Ref(_)
        | TypeShape::Expanded { .. }
        | TypeShape::Option(_)
        | TypeShape::Maybe(_)
        | TypeShape::Patch(_) => Ok(shape),
    }
}

fn transform_inline_shape(shape: InlineShape) -> Result<TypeShape> {
    let all_of = match shape {
        InlineShape::AllOf(ref inner) => inner,
        InlineShape::Struct(_) | InlineShape::Enum(_) | InlineShape::OneOf(_) => {
            return Ok(TypeShape::Inline(shape))
        }
    };
    let Some(ref_shape) = all_of.pop_if_only_one_ref()? else {
        return Ok(TypeShape::Inline(shape));
    };
    Ok(TypeShape::Ref(ref_shape))
}
