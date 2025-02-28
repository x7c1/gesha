use crate::misc::{MapOutput, TryMap};
use crate::v3_0::components::schemas::TypeShape::{Expanded, Inline};
use crate::v3_0::components::schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, EnumShape, FieldShape, ModShape, NewTypeShape,
    OneOfItemShape, OneOfShape, StructShape, TypeHeaderShape, TypePath, TypeShape,
};
use crate::v3_0::components::ComponentsShape;
use gesha_core::conversions::{by_key, Result};
use std::ops::Not;
use DefinitionShape::{AllOf, Enum, Mod, NewType, OneOf, Struct};

pub fn expand_inline_schemas(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    let defs = shape.schemas.root.defs;
    let defs = defs
        .map_output(expand)
        .to_result()?
        .into_iter()
        .flatten()
        .collect();

    shape.schemas.root.defs = defs;
    Ok(shape)
}

fn expand(def: DefinitionShape) -> Result<Vec<DefinitionShape>> {
    match def {
        Struct(x) => {
            let name = x.header.name.clone();
            expand_struct_fields(TypePath::new(), x).map_err(by_key(name))
        }
        AllOf(x) => {
            let name = x.header.name.clone();
            expand_all_of_fields(TypePath::new(), x).map_err(by_key(name))
        }
        OneOf(_) => {
            // inline definition in oneOf is not supported
            Ok(vec![def])
        }
        NewType(x) => {
            let name = x.header.name.clone();
            expand_newtype_field(TypePath::new(), x).map_err(by_key(name))
        }
        Enum(_) | Mod(_) => {
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

// return (newtype-shape, mod-shape)
fn expand_newtype_field(path: TypePath, mut shape: NewTypeShape) -> Result<Vec<DefinitionShape>> {
    let mod_name = shape.header.name.to_snake_case();
    let path = path.add(mod_name.clone());

    let (type_shape, defs) = match shape.type_shape {
        TypeShape::Array {
            type_shape,
            optionality,
        } => {
            let item_name = String::from(mod_name.clone()) + "_item";
            let (expanded, defs) = expand_type_shape(path, item_name, *type_shape)?;
            let shape = TypeShape::Array {
                type_shape: Box::new(expanded),
                optionality,
            };
            (shape, defs)
        }
        TypeShape::Inline { .. }
        | TypeShape::Proper { .. }
        | TypeShape::Ref { .. }
        | TypeShape::Expanded { .. }
        | TypeShape::Option(_)
        | TypeShape::Maybe(_)
        | TypeShape::Patch(_) => {
            // nop
            (shape.type_shape, vec![])
        }
    };
    shape.type_shape = type_shape;

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
    let (type_shape, defs) = expand_type_shape(mod_path, field.name.clone(), field.type_shape)?;
    let field_shape = FieldShape {
        name: field.name,
        type_shape,
    };
    Ok((field_shape, defs))
}

fn expand_type_shape(
    mod_path: TypePath,
    type_name: impl Into<String>,
    type_shape: TypeShape,
) -> Result<(TypeShape, Vec<DefinitionShape>)> {
    let Inline {
        object,
        optionality,
    } = type_shape
    else {
        return Ok((type_shape, vec![]));
    };
    let header = TypeHeaderShape::new(type_name, &object, vec![]);
    let type_name = header.name.clone();

    let defs = if let Some(cases) = object.all_of.as_ref() {
        let shape = AllOfShape {
            header,
            items: AllOfItemShape::from_schema_cases(cases.clone()).to_result()?,
            required: object.required,
        };
        expand_all_of_fields(mod_path.clone(), shape)?
    } else if let Some(cases) = object.one_of.as_ref() {
        vec![OneOf(OneOfShape {
            header,
            items: OneOfItemShape::from_schema_cases(cases.clone())?,
        })]
    } else if let Some(values) = object.enum_values.as_ref() {
        vec![Enum(EnumShape::new(header, values.clone()))]
    } else {
        let fields = FieldShape::from_object(object).to_result()?;
        expand_struct_fields(mod_path.clone(), StructShape { header, fields })?
    };
    let type_shape = Expanded {
        type_path: mod_path.add(type_name),
        optionality,
    };
    Ok((type_shape, defs))
}

fn collect<A, B>(pairs: Vec<(A, Vec<B>)>) -> (Vec<A>, Vec<B>) {
    let (mut ys1, mut ys2) = (vec![], vec![]);
    for (x, mut xs) in pairs {
        ys1.push(x);
        ys2.append(&mut xs);
    }
    (ys1, ys2)
}
