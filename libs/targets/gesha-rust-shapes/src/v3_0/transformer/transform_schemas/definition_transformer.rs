use crate::v3_0::components::schemas::DefinitionShape::{AllOf, Enum, Mod, NewType, OneOf, Struct};
use crate::v3_0::components::schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, InlineSchema, InlineShape,
    NewTypeShape, OneOfShape, Optionality, RefShape, StructShape, TypeHeaderShape, TypeShape,
};
use gesha_collections::seq::TryMap;
use gesha_core::broken;
use gesha_core::conversions::Result;

pub trait DefinitionTransformer {
    fn transform(def: DefinitionShape) -> Result<DefinitionShape> {
        let transformed = match def {
            Struct(shape) => Self::transform_struct(shape)?.into(),
            OneOf(shape) => Self::transform_one_of(shape)?,
            NewType(shape) => Self::transform_new_type(shape)?.into(),
            AllOf(shape) => Self::transform_all_of(shape)?,
            Enum(_) => {
                // enum has no shape to transform
                def
            }
            Mod(_) => return Err(broken!(def)),
        };
        Ok(transformed)
    }

    fn transform_one_of(shape: OneOfShape) -> Result<DefinitionShape> {
        Ok(shape.into())
    }

    fn transform_inline_one_of(schema: InlineSchema) -> Result<TypeShape> {
        Ok(InlineShape::OneOf(schema).into())
    }

    fn transform_all_of(mut shape: AllOfShape) -> Result<DefinitionShape> {
        shape.items = shape
            .items
            .transform_items(|x| Self::transform_all_of_item(x))?;

        Ok(shape.into())
    }

    fn transform_inline_all_of(mut shape: InlineSchema) -> Result<TypeShape> {
        shape.all_of = shape
            .all_of
            .transform_items(|x| Self::transform_all_of_item(x))?;

        Ok(InlineShape::AllOf(shape).into())
    }

    fn transform_struct(mut shape: StructShape) -> Result<StructShape> {
        shape.fields = shape.fields.try_map(|field| Self::transform_field(field))?;
        Ok(shape)
    }
    fn transform_inline_struct_shape(mut shape: InlineSchema) -> Result<TypeShape> {
        shape.fields = shape.fields.try_map(|field| Self::transform_field(field))?;
        Ok(InlineShape::Struct(shape).into())
    }

    fn transform_field(mut field: FieldShape) -> Result<FieldShape> {
        field.type_shape = Self::transform_type_shape(field.type_shape)?;
        Ok(field)
    }

    fn transform_type_shape(shape: TypeShape) -> Result<TypeShape> {
        match shape {
            TypeShape::Inline(shape) => Self::transform_inline_shape(*shape),

            TypeShape::Array {
                type_shape,
                optionality,
            } => Self::transform_array_shape(*type_shape, optionality),

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
            InlineShape::OneOf(inline) => Self::transform_inline_one_of(inline),
            InlineShape::AllOf(inline) => Self::transform_inline_all_of(inline),
            InlineShape::Struct(inline) => Self::transform_inline_struct_shape(inline),
            InlineShape::Enum(_) => Ok(shape.into()),
        }
    }

    fn transform_array_shape(shape: TypeShape, optionality: Optionality) -> Result<TypeShape> {
        let transformed = Self::transform_type_shape(shape)?;
        Ok(TypeShape::Array {
            type_shape: Box::new(transformed),
            optionality,
        })
    }

    fn transform_new_type(mut shape: NewTypeShape) -> Result<NewTypeShape> {
        shape.type_shape = Self::transform_type_shape(shape.type_shape)?;
        Ok(shape)
    }

    fn transform_all_of_item(item: AllOfItemShape) -> Result<AllOfItemShape> {
        let AllOfItemShape::Object(fields) = item else {
            return Ok(item);
        };
        let transformed = fields
            .into_iter()
            .map(|field| Self::transform_field(field))
            .collect::<Result<Vec<_>>>()?;

        Ok(AllOfItemShape::Object(transformed))
    }

    fn define_ref(mut ref_shape: RefShape, header: TypeHeaderShape) -> Result<DefinitionShape> {
        ref_shape.nullable = Some(header.is_nullable);
        let type_shape = TypeShape::from(ref_shape);
        let def_shape = NewTypeShape::new(header, type_shape);
        Ok(def_shape.into())
    }

    fn overwrite_ref(mut ref_shape: RefShape, optionality: &Optionality) -> Result<TypeShape> {
        ref_shape.is_required = optionality.is_required;
        ref_shape.nullable = Some(optionality.is_nullable);
        Ok(TypeShape::Ref(ref_shape))
    }
}
