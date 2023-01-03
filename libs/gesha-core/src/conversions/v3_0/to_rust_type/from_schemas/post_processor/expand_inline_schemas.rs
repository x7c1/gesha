use crate::conversions::v3_0::to_rust_type::from_schemas::to_field_shapes::to_field_shapes;
use crate::conversions::v3_0::to_rust_type::from_schemas::DefinitionShape::Mod;
use crate::conversions::v3_0::to_rust_type::from_schemas::TypeShape::{Expanded, Higher};
use crate::conversions::v3_0::to_rust_type::from_schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, PostProcessor, StructShape,
    TypeHeaderShape, TypePath, TypeShape,
};
use crate::conversions::Result;
use std::ops::Not;
use TypeShape::{Array, Fixed, InlineObject, Ref};

impl PostProcessor {
    pub fn process_inline_schemas(
        &self,
        shapes: Vec<DefinitionShape>,
    ) -> Result<Vec<DefinitionShape>> {
        let expanded_mod_shapes = shapes
            .into_iter()
            .map(|x| match x {
                DefinitionShape::Struct(x) => {
                    let (x1, x2) = expand_struct_fields(TypePath::new(), x)?;
                    Ok(vec![x1.into()].into_iter().chain(x2).collect())
                }
                DefinitionShape::AllOf(_)
                | DefinitionShape::NewType { .. }
                | DefinitionShape::Enum { .. }
                | Mod { .. } => Ok(vec![x]),
            })
            .collect::<Result<Vec<Vec<_>>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(expanded_mod_shapes)
    }
}

// return (struct-shape, mod-shape)
fn expand_struct_fields(
    path: TypePath,
    shape: StructShape,
) -> Result<(StructShape, Option<DefinitionShape>)> {
    let mod_name = shape.header.name.to_snake_case();
    let path = path.add(mod_name.clone());
    let expanded = shape
        .fields
        .into_iter()
        .map(|field| expand(path.clone(), field))
        .collect::<Result<Vec<_>>>()?;

    let (fields, defs) = collect(expanded);
    let next = StructShape {
        header: shape.header,
        fields,
    };
    let mod_def = defs.is_empty().not().then_some(Mod {
        name: mod_name,
        defs,
    });
    Ok((next, mod_def))
}

// return (all-of-shape, mod-shape)
fn expand_all_of_fields(
    path: TypePath,
    shape: AllOfShape,
) -> Result<(AllOfShape, Option<DefinitionShape>)> {
    let mod_name = shape.header.name.to_snake_case();
    let path = path.add(mod_name.clone());
    let expanded = shape
        .items
        .into_iter()
        .map(|x| match x {
            AllOfItemShape::Object(fields) => {
                let expanded_shapes = fields
                    .into_iter()
                    .map(|field| expand(path.clone(), field))
                    .collect::<Result<Vec<_>>>()?;

                let (fields, defs) = collect(expanded_shapes);
                Ok((AllOfItemShape::Object(fields), defs))
            }
            AllOfItemShape::Ref(_) => Ok((x, vec![])),
        })
        .collect::<Result<Vec<_>>>()?;

    let (items, defs) = collect(expanded);
    let next = AllOfShape {
        header: shape.header,
        items,
    };
    let mod_def = defs.is_empty().not().then_some(Mod {
        name: mod_name,
        defs,
    });
    Ok((next, mod_def))
}

fn expand(mod_path: TypePath, field: FieldShape) -> Result<(FieldShape, Vec<DefinitionShape>)> {
    match &field.type_shape {
        Ref { .. } | Fixed { .. } | Array { .. } | Expanded { .. } | Higher { .. } => {
            Ok((field, vec![]))
        }
        InlineObject {
            object,
            is_required,
            is_nullable,
        } => {
            let type_name = field.name.to_upper_camel_case();
            let (type_def, mod_def) = if let Some(cases) = object.all_of.as_ref() {
                let all_of_def = AllOfShape {
                    header: TypeHeaderShape::new(type_name.clone(), object),
                    items: AllOfItemShape::from_schema_cases(cases.clone())?,
                };
                let (shape, mod_def) = expand_all_of_fields(mod_path.clone(), all_of_def)?;
                (shape.into(), mod_def)
            } else {
                let struct_def = StructShape {
                    header: TypeHeaderShape::new(type_name.clone(), object),
                    fields: to_field_shapes(object.properties.clone(), object.required.clone())?,
                };
                let (shape, mod_def) = expand_struct_fields(mod_path.clone(), struct_def)?;
                (shape.into(), mod_def)
            };
            let field_shape = FieldShape {
                name: field.name,
                type_shape: Expanded {
                    type_path: mod_path.add(type_name),
                    is_required: *is_required,
                    is_nullable: *is_nullable,
                },
            };
            let defs = vec![type_def].into_iter().chain(mod_def).collect();
            Ok((field_shape, defs))
        }
    }
}

fn collect<A, B>(pairs: Vec<(A, Vec<B>)>) -> (Vec<A>, Vec<B>) {
    let (mut ys1, mut ys2) = (vec![], vec![]);
    for (x, mut xs) in pairs {
        ys1.push(x);
        ys2.append(&mut xs);
    }
    (ys1, ys2)
}
