use crate::misc::{MapOutput, TryMap};
use crate::v3_0::components::schemas::DefinitionShape::{AllOf, Mod};
use crate::v3_0::components::schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, InlineAllOfShape, InlineShape,
    InlineStructShape, NewTypeShape, StructShape, TypeShape,
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
fn transform_all_of(mut shape: AllOfShape) -> Result<DefinitionShape> {
    if let Some(ref_shape) = shape.pop_if_only_one_ref() {
        let type_shape = TypeShape::from(ref_shape);
        let def_shape = NewTypeShape::new(shape.header, type_shape);
        return Ok(def_shape.into());
    };
    shape.items = shape
        .items
        .into_iter()
        .map(transform_all_of_item)
        .collect::<Result<Vec<_>>>()?;

    Ok(shape.into())
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
    match shape {
        InlineShape::AllOf(inline) => transform_inline_all_of_shape(inline),
        InlineShape::Struct(inline) => transform_inline_struct_shape(inline),
        InlineShape::Enum(_) | InlineShape::OneOf(_) => Ok(shape.into()),
    }
}

fn transform_inline_struct_shape(mut shape: InlineStructShape) -> Result<TypeShape> {
    shape.object.fields = shape.object.fields.try_map(transform_field_shape)?;
    Ok(TypeShape::Inline(shape.into()))
}

fn transform_inline_all_of_shape(all_of: InlineAllOfShape) -> Result<TypeShape> {
    let Some(ref_shape) = all_of.pop_if_only_one_ref()? else {
        return Ok(TypeShape::Inline(InlineShape::AllOf(all_of)));
    };
    // TODO: convert other items
    Ok(TypeShape::Ref(ref_shape))
}

fn transform_all_of_item(item: AllOfItemShape) -> Result<AllOfItemShape> {
    let AllOfItemShape::Object(fields) = item else {
        return Ok(item);
    };
    let transformed = fields
        .into_iter()
        .map(transform_field_shape)
        .collect::<Result<Vec<_>>>()?;

    Ok(AllOfItemShape::Object(transformed))
}

fn transform_field_shape(shape: FieldShape) -> Result<FieldShape> {
    let FieldShape { name, type_shape } = shape;
    let type_shape = transform_type_shape(type_shape)?;
    Ok(FieldShape { name, type_shape })
}
