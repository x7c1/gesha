use crate::conversions::v3_0::to_rust_type::components_shapes::ComponentsShapes;
use crate::conversions::v3_0::to_rust_type::post_process::PostProcessor;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::{Fixed, InProcess};
use crate::conversions::v3_0::to_rust_type::{
    is_patch, DefinitionShape, FieldShape, PostProcess, TypeShape,
};
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;
use crate::targets::rust_type::{
    DataType, NewTypeDef, StructDef, StructField, StructFieldAttribute,
};

impl PostProcessor {
    pub(super) fn process_ref(&self, modules: &mut ComponentsShapes) -> Result<()> {
        // TODO: support other locations like "#/components/responses/" etc
        RefResolver::run("#/components/schemas/", &mut modules.schemas)
    }
}

struct RefResolver {
    prefix: &'static str,
}

impl RefResolver {
    fn run(prefix: &'static str, shapes: &mut [DefinitionShape]) -> Result<()> {
        let this = Self { prefix };
        shapes.iter_mut().try_for_each(|x| this.resolve_ref(x))
    }

    fn resolve_ref(&self, shape: &mut DefinitionShape) -> Result<()> {
        if let InProcess(process) = shape {
            *shape = self.shape_ref(process)?;
        };
        Ok(())
    }

    fn shape_ref(&self, process: &mut PostProcess) -> Result<DefinitionShape> {
        match process {
            PostProcess::Struct { header, shapes } => {
                let def = StructDef::new(header.clone(), self.shapes_to_fields(shapes)?);
                Ok(Fixed(def.into()))
            }
            PostProcess::NewType { header, type_shape } => {
                let def = NewTypeDef::new(header.clone(), self.type_shape_to_data_type(type_shape));
                Ok(Fixed(def.into()))
            }
            PostProcess::AllOf { .. } => Err(PostProcessBroken {
                detail: format!("'allOf' must be processed before '$ref'.\n{:#?}", process),
            }),
        }
    }

    fn shapes_to_fields(&self, shapes: &[FieldShape]) -> Result<Vec<StructField>> {
        let fields = shapes
            .iter()
            .map(|shape| self.field_shape_to_struct_field(shape))
            .collect::<Vec<StructField>>();

        Ok(fields)
    }

    fn field_shape_to_struct_field(&self, shape: &FieldShape) -> StructField {
        match shape {
            FieldShape::Fixed(x) => x.clone(),
            FieldShape::InProcess { name, type_shape } => {
                let field_name = name.clone();
                let data_type = self.type_shape_to_data_type(type_shape);

                let mut attributes = vec![];
                if let Some(original_name) = field_name.find_to_rename() {
                    attributes.push(StructFieldAttribute::new(format!(
                        r#"serde(rename="{original_name}")"#
                    )));
                }
                if is_patch(&data_type) {
                    attributes.push(StructFieldAttribute::new(
                        r#"serde(default, skip_serializing_if = "Patch::is_absent")"#,
                    ));
                }
                StructField::new(field_name, data_type, attributes)
            }
        }
    }

    fn type_shape_to_data_type(&self, shape: &TypeShape) -> DataType {
        let is_required = shape.is_required();
        let is_nullable = self.is_nullable(shape);
        let mut data_type = match shape {
            TypeShape::Fixed { data_type, .. } => data_type.clone(),
            TypeShape::Vec { type_shape, .. } => {
                DataType::Vec(Box::new(self.type_shape_to_data_type(&*type_shape)))
            }
            TypeShape::Ref { object, .. } => {
                let type_name = match String::from(object.clone()) {
                    x if x.starts_with(self.prefix) => x.replace(self.prefix, ""),
                    x => unimplemented!("not implemented: {x}"),
                };
                DataType::Custom(type_name)
            }
        };
        match (is_required, is_nullable) {
            (true, true) | (false, false) => {
                data_type = DataType::Option(Box::new(data_type));
            }
            (false, true) => {
                data_type = DataType::Patch(Box::new(data_type));
            }
            (true, false) => {
                // nop
            }
        }
        data_type
    }

    fn is_nullable(&self, shape: &TypeShape) -> bool {
        match shape {
            TypeShape::Fixed { is_nullable, .. } => *is_nullable,
            TypeShape::Vec { is_nullable, .. } => *is_nullable,
            TypeShape::Ref { .. } => {
                // TODO:
                false
            }
        }
    }
}
