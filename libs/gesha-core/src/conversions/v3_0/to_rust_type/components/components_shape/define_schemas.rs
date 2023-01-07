use crate::conversions::v3_0::to_rust_type::components::components_shape::create_module;
use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, FieldShape, ModShape, SchemasShape, StructShape, TypeHeaderShape, TypeShape,
};
use crate::conversions::v3_0::to_rust_type::is_patch;
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;
use crate::targets::rust_type::{
    DataType, Definition, Definitions, EnumDef, EnumVariant, EnumVariantAttribute, EnumVariantName,
    ModDef, ModuleName, NewTypeDef, Package, StructDef, StructField, StructFieldAttribute,
    StructFieldName, TypeHeader,
};
use openapi_types::v3_0::ComponentName;

pub fn define_schemas(shape: SchemasShape) -> Result<Option<ModDef>> {
    let define = Definer {};
    let defs = shape
        .into_iter()
        .map(|x| define.apply(x))
        .collect::<Result<Definitions>>()?;

    create_module("schemas", defs)
}

struct Definer {}

impl Definer {
    fn apply(&self, shape: DefinitionShape) -> Result<Definition> {
        match shape {
            DefinitionShape::Struct(StructShape { header, fields }) => {
                let def = StructDef::new(to_type_header(header), self.shapes_to_fields(fields)?);
                Ok(def.into())
            }
            DefinitionShape::NewType { header, type_shape } => {
                let def_type = type_shape_to_data_type(type_shape)?;
                let def = NewTypeDef::new(to_type_header(header.clone()), def_type);
                Ok(def.into())
            }
            DefinitionShape::Enum { header, values } => {
                let variants = values.into_iter().map(to_enum_variant).collect();
                let def = EnumDef::new(to_type_header(header.clone()), variants);
                Ok(def.into())
            }
            DefinitionShape::AllOf { .. } => Err(PostProcessBroken {
                detail: format!(
                    "'allOf' must be processed before 'to_definitions'.\n{:#?}",
                    shape
                ),
            }),
            DefinitionShape::Mod(ModShape { name, defs }) => {
                let inline_defs = defs
                    .into_iter()
                    .map(|x| self.apply(x))
                    .collect::<Result<Vec<_>>>()?;

                let def = ModDef {
                    name: ModuleName::new(name.clone()),
                    imports: vec![Package::Deserialize, Package::Serialize].into(),
                    defs: inline_defs.into_iter().collect(),
                };
                Ok(def.into())
            }
        }
    }

    fn shapes_to_fields(&self, shapes: Vec<FieldShape>) -> Result<Vec<StructField>> {
        shapes
            .into_iter()
            .map(|shape| self.field_shape_to_struct_field(shape))
            .collect()
    }

    fn field_shape_to_struct_field(&self, shape: FieldShape) -> Result<StructField> {
        let data_type = type_shape_to_data_type(shape.type_shape)?;
        let name = StructFieldName::new(shape.name.as_ref());
        let attrs = to_field_attrs(&shape.name, &name, &data_type);
        let field = StructField::new(name, data_type, attrs);
        Ok(field)
    }
}
fn type_shape_to_data_type(shape: TypeShape) -> Result<DataType> {
    let data_type = match shape {
        TypeShape::Array { type_shape, .. } => {
            DataType::Vec(Box::new(type_shape_to_data_type(*type_shape)?))
        }
        TypeShape::Ref { .. } => {
            todo!()
        }
        TypeShape::Fixed { data_type, .. } => data_type.clone(),
        TypeShape::InlineObject { .. } => Err(PostProcessBroken {
            detail: format!(
                "InlineObject must be processed before '$ref'.\n{:#?}",
                shape
            ),
        })?,
        TypeShape::Expanded { type_path, .. } => type_path.clone().into(),
        TypeShape::Option(type_shape) => {
            DataType::Option(Box::new(type_shape_to_data_type(*type_shape)?))
        }
        TypeShape::Patch(type_shape) => {
            DataType::Patch(Box::new(type_shape_to_data_type(*type_shape)?))
        }
    };
    Ok(data_type)
}

fn to_field_attrs(
    original: &ComponentName,
    name: &StructFieldName,
    tpe: &DataType,
) -> Vec<StructFieldAttribute> {
    let mut attrs = vec![];
    if original.as_ref() != name.as_str() {
        attrs.push(StructFieldAttribute::new(format!(
            r#"serde(rename="{original}")"#
        )));
    }
    if is_patch(tpe) {
        attrs.push(StructFieldAttribute::new(
            r#"serde(default, skip_serializing_if = "Patch::is_absent")"#,
        ));
    }
    attrs
}

fn to_type_header(shape: TypeHeaderShape) -> TypeHeader {
    TypeHeader::new(shape.name, shape.doc_comments)
}

fn to_enum_variant(original: String) -> EnumVariant {
    let name = EnumVariantName::new(original.as_str());
    let mut attrs = vec![];
    if name.as_str() != original {
        attrs.push(EnumVariantAttribute::new(format!(
            r#"serde(rename="{original}")"#
        )))
    }
    EnumVariant::unit(name, attrs)
}
