use crate::conversions::v3_0::to_rust_type::from_schemas::to_field_shapes::to_field_shapes;
use crate::conversions::v3_0::to_rust_type::from_schemas::DefinitionShape::Mod;
use crate::conversions::v3_0::to_rust_type::from_schemas::TypeShape::Expanded;
use crate::conversions::v3_0::to_rust_type::from_schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, PostProcessor, StructShape,
    TypeHeaderShape, TypePath, TypeShape,
};
use crate::conversions::Result;
use TypeShape::{Array, Fixed, InlineObject, Ref};

impl PostProcessor {
    pub fn process_inline_schemas(&self, shapes: &mut Vec<DefinitionShape>) -> Result<()> {
        let mut expanded_mod_shapes = shapes
            .iter_mut()
            .filter_map(|x| x.as_mut_struct())
            .map(|x| expand_struct_fields(TypePath::new(), x))
            .collect::<Result<Vec<Option<_>>>>()?
            .into_iter()
            .flatten()
            .collect();

        shapes.append(&mut expanded_mod_shapes);
        Ok(())
    }
}

fn expand_struct_fields(
    path: TypePath,
    shape: &mut StructShape,
) -> Result<Option<DefinitionShape>> {
    let mod_name = shape.header.name.to_snake_case();
    let path = path.add(mod_name.clone());
    let expanded_shapes = shape
        .fields
        .iter_mut()
        .map(|field| expand(path.clone(), field))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    if expanded_shapes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Mod {
            name: mod_name,
            defs: expanded_shapes,
        }))
    }
}

fn expand_all_of_fields(path: TypePath, shape: &mut AllOfShape) -> Result<Option<DefinitionShape>> {
    let mod_name = shape.header.name.to_snake_case();
    let path = path.add(mod_name.clone());
    let expanded_shapes = shape
        .fields()
        .map(|field| expand(path.clone(), field))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    if expanded_shapes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Mod {
            name: mod_name,
            defs: expanded_shapes,
        }))
    }
}

fn expand(mod_path: TypePath, field: &mut FieldShape) -> Result<Vec<DefinitionShape>> {
    match &field.type_shape {
        Ref { .. } | Fixed { .. } | Array { .. } | Expanded { .. } => Ok(vec![]),
        InlineObject {
            object,
            is_required,
            is_nullable,
        } => {
            let type_name = field.name.to_upper_camel_case();
            let (type_def, mod_def) = if let Some(cases) = object.all_of.as_ref() {
                let mut all_of_def = AllOfShape {
                    header: TypeHeaderShape::new(type_name.clone(), object),
                    items: AllOfItemShape::from_schema_cases(cases.clone())?,
                };
                let mod_def = expand_all_of_fields(mod_path.clone(), &mut all_of_def)?;
                (all_of_def.into(), mod_def)
            } else {
                let mut struct_def = StructShape {
                    header: TypeHeaderShape::new(type_name.clone(), object),
                    fields: to_field_shapes(object.properties.clone(), object.required.clone())?,
                };
                let mod_def = expand_struct_fields(mod_path.clone(), &mut struct_def)?;
                (struct_def.into(), mod_def)
            };
            field.type_shape = Expanded {
                type_path: mod_path.add(type_name),
                is_required: *is_required,
                is_nullable: *is_nullable,
            };
            let defs = vec![type_def].into_iter().chain(mod_def).collect();
            Ok(defs)
        }
    }
}
