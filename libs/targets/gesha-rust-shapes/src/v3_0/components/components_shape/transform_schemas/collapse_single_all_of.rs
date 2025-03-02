use crate::misc::MapOutput;
use crate::v3_0::components::schemas::DefinitionShape::{AllOf, Mod};
use crate::v3_0::components::schemas::{
    AllOfShape, DefinitionShape, FieldShape, NewTypeShape, RefShape, StructShape, TypeShape,
};
use crate::v3_0::components::ComponentsShape;
use gesha_core::broken;
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaCase;
use tracing::error;
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
        AllOf(shape) => transform_all_of(shape)?.into(),
        NewType(shape) => transform_new_type(shape)?.into(),
        Enum(_) => {
            // enum has no shape to transform
            def
        }
        OneOf(ref shape) => {
            def
        }
        Mod(_) => return Err(broken!(def)),
    };
    Ok(transformed)
}

fn transform_struct(shape: StructShape) -> Result<StructShape> {
    todo!()
}

fn transform_all_of(shape: AllOfShape) -> Result<AllOfShape> {
    todo!()
}

fn transform_new_type(shape: NewTypeShape) -> Result<NewTypeShape> {
    todo!()
}

/*
fn transform_struct(shape: StructShape) -> Result<StructShape> {
    let fields = shape.fields;
    let next = StructShape {
        header: shape.header,
        fields: fields
            .into_iter()
            .map(transform_field)
            .collect::<Result<Vec<_>>>()?,
    };
    Ok(next.into())
}

fn transform_field(mut field: FieldShape) -> Result<FieldShape> {
    match &field.type_shape {
        TypeShape::Inline {
            object,
            optionality,
        } => {
            let single_ref = object
                .all_of
                .as_ref()
                .and_then(|x| (x.len() == 1).then_some(&x[0]))
                .and_then(|x| match x {
                    SchemaCase::Schema(_) => None,
                    SchemaCase::Reference(x) => Some(x),
                });

            if let Some(rf) = single_ref {
                /*
                // TODO: support optionality.is_nullable
                for following code
                ```
                nullable: true
                allOf:
                  - $ref: '#/components/schemas/FooBar'
                ```
                */
                field.type_shape = RefShape::new(rf.clone(), optionality.is_required)?.into();
                return Ok(field);
            };

            Ok(field)
        }
        TypeShape::Proper { .. }
        | TypeShape::Array { .. }
        | TypeShape::Ref(_)
        | TypeShape::Expanded { .. }
        | TypeShape::Option(_)
        | TypeShape::Maybe(_)
        | TypeShape::Patch(_) => Ok(field),
    }
}
*/
