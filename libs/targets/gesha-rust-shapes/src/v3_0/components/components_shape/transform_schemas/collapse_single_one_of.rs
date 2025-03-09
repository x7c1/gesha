use crate::misc::{MapOutput, TryMap};
use crate::v3_0::components::schemas::DefinitionShape::{AllOf, Mod};
use crate::v3_0::components::schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, InlineSchema, InlineShape,
    NewTypeShape, OneOfShape, Optionality, StructShape, TypeShape,
};
use crate::v3_0::components::ComponentsShape;
use gesha_core::broken;
use gesha_core::conversions::Result;
use DefinitionShape::{Enum, NewType, OneOf, Struct};

/// If `oneOf` has only one $ref,
/// replace it with a Shape containing a single `$ref`.
pub fn collapse_single_one_of(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shape.schemas.root.defs;
    shape.schemas.root.defs = defs.map_output(transform).to_result()?;
    Ok(shape)
}

fn transform(def: DefinitionShape) -> Result<DefinitionShape> {
    let transformed = match def {
        Struct(shape) => transform_struct(shape)?.into(),
        OneOf(shape) => transform_one_of(shape)?,
        NewType(shape) => transform_new_type(shape)?.into(),
        AllOf(shape) => transform_all_of(shape)?.into(),
        Enum(_) => {
            // enum has no shape to transform
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

/// return NewTypeShape if given OneOfShape has only one $ref
fn transform_one_of(shape: OneOfShape) -> Result<DefinitionShape> {
    if let Some(mut ref_shape) = shape.items.head_if_single_ref().cloned() {
        ref_shape.nullable = Some(shape.header.is_nullable);

        let type_shape = TypeShape::from(ref_shape);
        let def_shape = NewTypeShape::new(shape.header, type_shape);
        return Ok(def_shape.into());
    };
    Ok(shape.into())
}

fn transform_all_of(mut shape: AllOfShape) -> Result<AllOfShape> {
    shape.items = shape.items.transform_items(transform_all_of_item)?;
    Ok(shape)
}

fn transform_all_of_item(item: AllOfItemShape) -> Result<AllOfItemShape> {
    let AllOfItemShape::Object(fields) = item else {
        return Ok(item);
    };
    let transformed = fields
        .into_iter()
        .map(transform_field)
        .collect::<Result<Vec<_>>>()?;

    Ok(AllOfItemShape::Object(transformed))
}

fn transform_new_type(mut shape: NewTypeShape) -> Result<NewTypeShape> {
    shape.type_shape = transform_type_shape(shape.type_shape)?;
    Ok(shape)
}

fn transform_field(mut field: FieldShape) -> Result<FieldShape> {
    field.type_shape = transform_type_shape(field.type_shape)?;
    Ok(field)
}

fn transform_type_shape(shape: TypeShape) -> Result<TypeShape> {
    match shape {
        TypeShape::Inline(shape) => transform_inline_shape(*shape),

        TypeShape::Array {
            type_shape,
            optionality,
        } => transform_array_shape(*type_shape, optionality),

        TypeShape::Proper { .. }
        | TypeShape::Ref(_)
        | TypeShape::Expanded { .. }
        | TypeShape::Option(_)
        | TypeShape::Maybe(_)
        | TypeShape::Patch(_) => Ok(shape),
    }
}

fn transform_inline_shape(shape: InlineShape) -> Result<TypeShape> {
    match shape {
        InlineShape::OneOf(inline) => transform_inline_one_of_shape(inline),
        InlineShape::AllOf(inline) => transform_inline_all_of_shape(inline),
        InlineShape::Struct(inline) => transform_inline_struct_shape(inline),
        InlineShape::Enum(_) => Ok(shape.into()),
    }
}

fn transform_array_shape(shape: TypeShape, optionality: Optionality) -> Result<TypeShape> {
    let transformed = transform_type_shape(shape)?;
    Ok(TypeShape::Array {
        type_shape: Box::new(transformed),
        optionality,
    })
}

fn transform_inline_struct_shape(mut shape: InlineSchema) -> Result<TypeShape> {
    shape.fields = shape.fields.try_map(transform_field)?;
    Ok(InlineShape::Struct(shape).into())
}

fn transform_inline_all_of_shape(mut shape: InlineSchema) -> Result<TypeShape> {
    shape.all_of = shape.all_of.transform_items(transform_all_of_item)?;
    Ok(InlineShape::AllOf(shape).into())
}

fn transform_inline_one_of_shape(schema: InlineSchema) -> Result<TypeShape> {
    if let Some(mut ref_shape) = schema.one_of.head_if_single_ref().cloned() {
        ref_shape.is_required = schema.optionality.is_required;
        ref_shape.nullable = Some(schema.optionality.is_nullable);
        return Ok(TypeShape::Ref(ref_shape));
    };
    Ok(InlineShape::OneOf(schema).into())
}
