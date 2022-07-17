use crate::conversions::v3_0::to_rust_type::components_shapes::ComponentsShapes;
use crate::conversions::v3_0::to_rust_type::post_processor::PostProcessor;
use crate::conversions::v3_0::to_rust_type::{
    is_patch, DefinitionShape, FieldShape, TypeHeaderShape, TypeShape,
};
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;
use crate::targets::rust_type::{
    DataType, Definition, EnumDef, EnumVariant, NewTypeDef, StructDef, StructField,
    StructFieldAttribute, StructFieldName, TypeHeader,
};

impl PostProcessor {
    pub(super) fn process_ref(&self, modules: &mut ComponentsShapes) -> Result<Vec<Definition>> {
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
    ) -> Result<Vec<Definition>> {
        let this = RefResolver { prefix, original };
        shapes.iter_mut().map(|x| this.resolve_ref(x)).collect()
    }

    fn resolve_ref(&self, shape: &mut DefinitionShape) -> Result<Definition> {
        match shape {
            DefinitionShape::Struct { header, shapes } => {
                let def = StructDef::new(
                    to_type_header(header.clone()),
                    self.shapes_to_fields(shapes)?,
                );
                Ok(def.into())
            }
            DefinitionShape::NewType { header, type_shape } => {
                let def_type = self.type_shape_to_data_type(type_shape)?;
                let def = NewTypeDef::new(to_type_header(header.clone()), def_type);
                Ok(def.into())
            }
            DefinitionShape::Enum { header, values } => {
                let variants = Clone::clone(values)
                    .into_iter()
                    .map(EnumVariant::new)
                    .collect();

                let def = EnumDef::new(to_type_header(header.clone()), variants);
                Ok(def.into())
            }
            DefinitionShape::AllOf { .. } => Err(PostProcessBroken {
                detail: format!("'allOf' must be processed before '$ref'.\n{:#?}", shape),
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
        let data_type = self.type_shape_to_data_type(&shape.type_shape)?;
        let attrs = to_field_attrs(&shape.name, &data_type);
        let field = StructField::new(shape.name.clone(), data_type, attrs);
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

fn to_type_header(shape: TypeHeaderShape) -> TypeHeader {
    TypeHeader::new(shape.name, shape.doc_comments)
}
