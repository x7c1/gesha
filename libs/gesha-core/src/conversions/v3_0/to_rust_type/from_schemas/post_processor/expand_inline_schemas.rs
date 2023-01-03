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
        let defs = shapes
            .into_iter()
            .map(expand)
            .collect::<Result<Vec<Vec<_>>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(defs)
    }
}

fn expand(shape: DefinitionShape) -> Result<Vec<DefinitionShape>> {
    match shape {
        DefinitionShape::Struct(x) => {
            let (struct_shape, mod_shape) = expand_struct_fields(TypePath::new(), x)?;
            Ok(vec![struct_shape.into()].into_iter().chain(mod_shape).collect())
        }
        DefinitionShape::AllOf(_)// TODO: add test
        | DefinitionShape::NewType { .. }
        | DefinitionShape::Enum { .. }
        | Mod { .. } => Ok(vec![shape]),
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
        .map(|field| expand_field(path.clone(), field))
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
        .map(|x| x.expand_fields(expand_fields_from(&path)))
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

fn expand_fields_from(
    path: &TypePath,
) -> impl Fn(Vec<FieldShape>) -> Result<(Vec<FieldShape>, Vec<DefinitionShape>)> + '_ {
    move |fields| {
        let expanded = fields
            .into_iter()
            .map(|field| expand_field(path.clone(), field))
            .collect::<Result<Vec<_>>>()?;

        Ok(collect(expanded))
    }
}

fn expand_field(
    mod_path: TypePath,
    field: FieldShape,
) -> Result<(FieldShape, Vec<DefinitionShape>)> {
    match field.type_shape {
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
                    header: TypeHeaderShape::new(type_name.clone(), &object),
                    items: AllOfItemShape::from_schema_cases(cases.clone())?,
                };
                let (shape, mod_def) = expand_all_of_fields(mod_path.clone(), all_of_def)?;
                (shape.into(), mod_def)
            } else {
                let struct_def = StructShape {
                    header: TypeHeaderShape::new(type_name.clone(), &object),
                    fields: to_field_shapes(object.properties.clone(), object.required.clone())?,
                };
                let (shape, mod_def) = expand_struct_fields(mod_path.clone(), struct_def)?;
                (shape.into(), mod_def)
            };
            let field_shape = FieldShape {
                name: field.name,
                type_shape: Expanded {
                    type_path: mod_path.add(type_name),
                    is_required,
                    is_nullable,
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
