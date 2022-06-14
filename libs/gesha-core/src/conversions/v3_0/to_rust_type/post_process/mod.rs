use super::{AllOfItemShape, ComponentsShapes, DefinitionShape, FieldShape, PostProcess};
use crate::conversions::v3_0::to_rust_type::TypeShape;
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, Definition, NewTypeDef, StructDef, StructField};
use openapi_types::v3_0::ReferenceObject;
use DefinitionShape::{Fixed, InProcess};

pub(super) struct PostProcessor {
    original: ComponentsShapes,
}

impl PostProcessor {
    pub fn run(modules: &mut ComponentsShapes) -> Result<()> {
        let this = Self {
            original: modules.clone(),
        };
        modules.schemas.iter_mut().try_for_each(|x| this.replace(x))
    }

    fn replace(&self, shape: &mut DefinitionShape) -> Result<()> {
        if let InProcess(process) = shape {
            *shape = self.shape_process(process)?;
        };
        Ok(())
    }

    fn shape_process(&self, process: &mut PostProcess) -> Result<DefinitionShape> {
        match process {
            PostProcess::AllOf {
                struct_name,
                shapes,
            } => {
                let def = StructDef {
                    name: struct_name.clone(),
                    fields: self.merge_fields_all_of(shapes)?,
                };
                Ok(Fixed(def.into()))
            }
            PostProcess::Struct {
                struct_name,
                shapes,
            } => {
                let def = StructDef {
                    name: struct_name.clone(),
                    fields: self.ref_to_fields(shapes)?,
                };
                Ok(Fixed(def.into()))
            }
            PostProcess::NewType {
                struct_name,
                type_shape,
            } => {
                let def = NewTypeDef {
                    name: struct_name.clone(),
                    data_type: self.reify_type_shape(type_shape),
                };
                Ok(Fixed(def.into()))
            }
        }
    }
}

// resolve '$ref'
impl PostProcessor {
    fn ref_to_fields(&self, shapes: &[FieldShape]) -> Result<Vec<StructField>> {
        let fields = shapes
            .iter()
            .map(|shape| self.resolve_ref(shape))
            .collect::<Vec<StructField>>();

        Ok(fields)
    }

    fn resolve_ref(&self, shape: &FieldShape) -> StructField {
        match shape {
            FieldShape::Fixed(x) => x.clone(),
            FieldShape::InProcess {
                name,
                type_shape,
                is_optional,
            } => {
                let mut data_type = self.reify_type_shape(type_shape);
                if *is_optional {
                    data_type = DataType::Option(Box::new(data_type));
                }
                StructField {
                    name: name.clone(),
                    data_type,
                }
            }
        }
    }

    fn reify_type_shape(&self, shape: &TypeShape) -> DataType {
        match shape {
            TypeShape::Fixed(x) => x.clone(),
            TypeShape::Vec(x) => DataType::Vec(Box::new(self.reify_type_shape(&*x))),
            TypeShape::Ref(x) => {
                let type_name = match String::from(x.clone()) {
                    x if x.starts_with("#/components/schemas/") => {
                        // TODO: change location to relative paths by checking self.original
                        // if using "#/components/responses/" etc
                        x.replace("#/components/schemas/", "")
                    }
                    x => unimplemented!("not implemented: {x}"),
                };
                DataType::Custom(type_name)
            }
        }
    }
}

// resolve 'allOf'
impl PostProcessor {
    fn merge_fields_all_of(&self, shapes: &[AllOfItemShape]) -> Result<Vec<StructField>> {
        let fields = shapes
            .iter()
            .flat_map(|shape| self.shape_to_fields(shape))
            .collect::<Vec<StructField>>();

        Ok(fields)
    }

    fn shape_to_fields(&self, item_shape: &AllOfItemShape) -> Vec<StructField> {
        let to_field = |shape: &FieldShape| match shape {
            FieldShape::Fixed(field) => field.clone(),
            FieldShape::InProcess {
                name,
                type_shape,
                is_optional,
            } => {
                unimplemented!("{} {:?} {}", name, type_shape, is_optional)
            }
        };
        match item_shape {
            AllOfItemShape::Object(shapes) => shapes.iter().map(to_field).collect(),
            AllOfItemShape::Ref(object) => self.find_struct_fields(object),
        }
    }

    fn find_struct_fields(&self, x: &ReferenceObject) -> Vec<StructField> {
        // TODO: support locations other than 'schemas'
        let prefix = "#/components/schemas/";
        let type_ref = x.as_ref();
        if type_ref.starts_with(prefix) {
            let name = type_ref.replace(prefix, "");
            self.traverse(&name, &self.original.schemas)
        } else {
            unimplemented!()
        }
    }

    fn traverse(&self, name: &str, defs: &[DefinitionShape]) -> Vec<StructField> {
        let xs = defs.iter().find_map(|def_shape| match def_shape {
            Fixed(def) => match def {
                Definition::StructDef(x) if x.name == name => Some(x.fields.clone()),
                _ => None,
            },
            InProcess(_) => unimplemented!(),
        });
        xs.unwrap_or_else(|| unimplemented!())
    }
}
