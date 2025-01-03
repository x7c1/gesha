use crate::misc::TryMap;
use crate::v3_0::components::schemas::TypeShape::{Expanded, Inline};
use crate::v3_0::components::schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, EnumShape, FieldShape, ModShape, OneOfItemShape,
    OneOfShape, StructShape, TypeHeaderShape, TypePath,
};
use crate::v3_0::components::ComponentsShape;
use crate::Result;
use std::ops::Not;
use DefinitionShape::{AllOf, Enum, Mod, NewType, OneOf, Struct};

pub fn expand_inline_schemas(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shape.schemas.root.defs;
    let defs = defs
        .into_iter()
        .map(expand)
        .collect::<Result<Vec<Vec<_>>>>()?
        .into_iter()
        .flatten()
        .collect();

    shape.schemas.root.defs = defs;
    Ok(shape)
}

fn expand(def: DefinitionShape) -> Result<Vec<DefinitionShape>> {
    match def {
        Struct(x) => expand_struct_fields(TypePath::new(), x),
        AllOf(x) => expand_all_of_fields(TypePath::new(), x),
        OneOf(_) => {
            // inline definition in oneOf is not supported
            Ok(vec![def])
        }
        NewType { .. } | Enum(_) | Mod(_) => {
            // nop
            Ok(vec![def])
        }
    }
}

// return (struct-shape, mod-shape)
fn expand_struct_fields(path: TypePath, mut shape: StructShape) -> Result<Vec<DefinitionShape>> {
    let mod_name = shape.header.name.to_snake_case();
    let path = path.add(mod_name.clone());
    let expanded = shape
        .fields
        .try_map(|field| expand_field(path.clone(), field))?;

    let (fields, defs) = collect(expanded);
    shape.fields = fields;

    let mod_def = defs
        .is_empty()
        .not()
        .then_some(ModShape::new(mod_name, defs).into());

    Ok(vec![shape.into()].into_iter().chain(mod_def).collect())
}

// return (all-of-shape, mod-shape)
fn expand_all_of_fields(path: TypePath, mut shape: AllOfShape) -> Result<Vec<DefinitionShape>> {
    let mod_name = shape.header.name.to_snake_case();
    let path = path.add(mod_name.clone());
    let expanded = shape
        .items
        .try_map(|x| x.expand_fields(expand_fields_from(&path)))?;

    let (items, defs) = collect(expanded);
    shape.items = items;

    let mod_def = defs
        .is_empty()
        .not()
        .then_some(ModShape::new(mod_name, defs).into());

    Ok(vec![shape.into()].into_iter().chain(mod_def).collect())
}

fn expand_fields_from(
    path: &TypePath,
) -> impl Fn(Vec<FieldShape>) -> Result<(Vec<FieldShape>, Vec<DefinitionShape>)> + '_ {
    |fields| {
        let expanded = fields.try_map(|field| expand_field(path.clone(), field))?;
        Ok(collect(expanded))
    }
}

fn expand_field(
    mod_path: TypePath,
    field: FieldShape,
) -> Result<(FieldShape, Vec<DefinitionShape>)> {
    let Inline {
        object,
        optionality,
    } = field.type_shape
    else {
        return Ok((field, vec![]));
    };
    let header = TypeHeaderShape::new(field.name.clone(), &object, vec![]);
    let type_name = header.name.clone();

    let defs = if let Some(cases) = object.all_of.as_ref() {
        expand_all_of_fields(
            mod_path.clone(),
            AllOfShape {
                header,
                items: AllOfItemShape::from_schema_cases(cases.clone())?,
                required: object.required,
            },
        )?
    } else if let Some(cases) = object.one_of.as_ref() {
        vec![OneOf(OneOfShape {
            header,
            items: OneOfItemShape::from_schema_cases(cases.clone())?,
        })]
    } else if let Some(values) = object.enum_values.as_ref() {
        vec![Enum(EnumShape::new(header, values.clone()))]
    } else {
        expand_struct_fields(
            mod_path.clone(),
            StructShape {
                header,
                fields: FieldShape::from_object(object)?,
            },
        )?
    };
    let field_shape = FieldShape {
        name: field.name,
        type_shape: Expanded {
            type_path: mod_path.add(type_name),
            optionality,
        },
    };
    Ok((field_shape, defs))
}

fn collect<A, B>(pairs: Vec<(A, Vec<B>)>) -> (Vec<A>, Vec<B>) {
    let (mut ys1, mut ys2) = (vec![], vec![]);
    for (x, mut xs) in pairs {
        ys1.push(x);
        ys2.append(&mut xs);
    }
    (ys1, ys2)
}
