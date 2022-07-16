use crate::conversions::v3_0::to_rust_type::components_shapes::ComponentsShapes;
use crate::conversions::v3_0::to_rust_type::post_process::PostProcessor;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::{Fixed, InProcess};
use crate::conversions::v3_0::to_rust_type::{
    is_patch, DefinitionShape, FieldShape, PostProcess, TypeShape,
};
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;
use crate::targets::rust_type::{
    DataType, NewTypeDef, StructDef, StructField, StructFieldAttribute, StructFieldName,
};

impl PostProcessor {
    pub(super) fn process_ref(&self, modules: &mut ComponentsShapes) -> Result<()> {
        // TODO: support other locations like "#/components/responses/" etc
        RefResolver::run(
            "#/components/schemas/",
            &mut modules.schemas,
            &self.original,
        )
    }
}

struct RefResolver<'a> {
    prefix: &'static str,
    original: &'a ComponentsShapes,
}

impl RefResolver<'_> {
    fn run(
        prefix: &'static str,
        shapes: &mut [DefinitionShape],
        original: &ComponentsShapes,
    ) -> Result<()> {
        let this = RefResolver { prefix, original };
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
                let def_type = self.type_shape_to_data_type(type_shape)?;
                let def = NewTypeDef::new(header.clone(), def_type);
                Ok(Fixed(def.into()))
            }
            PostProcess::AllOf { .. } => Err(PostProcessBroken {
                detail: format!("'allOf' must be processed before '$ref'.\n{:#?}", process),
            }),
        }
    }

    fn shapes_to_fields(&self, shapes: &[FieldShape]) -> Result<Vec<StructField>> {
        shapes
            .iter()
            .map(|shape| self.field_shape_to_struct_field(shape))
            .collect()
    }

    fn field_shape_to_struct_field(&self, shape: &FieldShape) -> Result<StructField> {
        let field = match shape {
            FieldShape::InProcess { name, type_shape } => {
                let data_type = self.type_shape_to_data_type(type_shape)?;
                let attrs = to_field_attrs(name, &data_type);
                StructField::new(name.clone(), data_type, attrs)
            }
            FieldShape::Fixed(x) => x.clone(),
        };
        Ok(field)
    }

    fn type_shape_to_data_type(&self, shape: &TypeShape) -> Result<DataType> {
        let is_required = shape.is_required();
        let is_nullable = self.is_nullable(shape)?;
        let mut data_type = match shape {
            TypeShape::Vec { type_shape, .. } => {
                DataType::Vec(Box::new(self.type_shape_to_data_type(&*type_shape)?))
            }
            TypeShape::Ref { object, .. } => {
                let type_name = match String::from(object.clone()) {
                    x if x.starts_with(self.prefix) => x.replace(self.prefix, ""),
                    x => unimplemented!("not implemented: {x}"),
                };
                DataType::Custom(type_name)
            }
            TypeShape::Fixed { data_type, .. } => data_type.clone(),
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
        Ok(data_type)
    }

    fn is_nullable(&self, shape: &TypeShape) -> Result<bool> {
        match shape {
            TypeShape::Fixed { is_nullable, .. } => Ok(*is_nullable),
            TypeShape::Vec { is_nullable, .. } => Ok(*is_nullable),
            TypeShape::Ref { object, .. } => self
                .original
                .find_definition(object)
                .map(|def| def.is_nullable()),
        }
    }
}

fn to_field_attrs(name: &StructFieldName, tpe: &DataType) -> Vec<StructFieldAttribute> {
    let mut attributes = vec![];
    if let Some(original) = name.find_to_rename() {
        attributes.push(StructFieldAttribute::new(format!(
            r#"serde(rename="{original}")"#
        )));
    }
    if is_patch(tpe) {
        attributes.push(StructFieldAttribute::new(
            r#"serde(default, skip_serializing_if = "Patch::is_absent")"#,
        ));
    }
    attributes
}
