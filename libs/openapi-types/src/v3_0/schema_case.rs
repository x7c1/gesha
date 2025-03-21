use crate::v3_0::all_of::AllOf;
use crate::v3_0::array_items::ArrayItems;
use crate::v3_0::{
    EnumValues, FormatModifier, OneOf, OpenApiDataType, ReferenceObject, RequiredSchemaFields,
    SchemaProperties,
};
use crate::yaml::YamlMap;
use crate::{Error, Output, Result};

/// Schema Object | Reference Object
#[derive(Clone, Debug)]
pub enum SchemaCase {
    Schema(Box<SchemaObject>),
    Reference(ReferenceObject<SchemaObject>),
}

impl SchemaCase {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<SchemaCase> {
        let case = match map.remove_if_exists::<String>("$ref")? {
            Some(rf) => {
                let reference = ReferenceObject::new(rf);
                SchemaCase::Reference(reference)
            }
            None => {
                let object = Self::to_schema_object(map)?;
                SchemaCase::Schema(Box::new(object))
            }
        };
        Ok(case)
    }

    fn to_schema_object(mut map: YamlMap) -> Result<SchemaObject> {
        let (properties, errors_of_properties) = map
            .flat_extract_if_exists("properties", SchemaProperties::from_yaml_map)
            .into_tuple();

        let (required, errors_of_required) = map
            .try_extract_if_exists("required", RequiredSchemaFields::from_yaml_array)
            .into_tuple();

        let (data_type, errors_of_data_type) = map
            .try_extract_if_exists("type", OpenApiDataType::new)
            .into_tuple();

        let (format, errors_of_format) = map
            .try_extract_if_exists("format", FormatModifier::from_string)
            .into_tuple();

        let (nullable, errors_of_nullable) = map.extract_if_exists("nullable").into_tuple();

        let (items, errors_of_items) = map
            .try_extract_if_exists("items", ArrayItems::from_yaml_map)
            .into_tuple();

        let (enum_values, errors_of_enum) = map
            .try_extract_if_exists("enum", EnumValues::from_yaml_array)
            .into_tuple();

        let (all_of, errors_all_of) = map
            .flat_extract_if_exists("allOf", AllOf::from_yaml_array)
            .into_tuple();

        let (one_of, errors_one_of) = map
            .flat_extract_if_exists("oneOf", OneOf::from_yaml_array)
            .into_tuple();

        let object = SchemaObject {
            title: map.remove_if_exists::<String>("title")?,
            description: map.remove_if_exists::<String>("description")?,
            data_type,
            format,
            nullable,
            properties,
            required,
            items,
            enum_values,
            all_of: all_of.flatten(),
            one_of: one_of.flatten(),
        };
        let output = Output::ok(object)
            .append(errors_of_properties)
            .append(errors_of_required)
            .append(errors_of_data_type)
            .append(errors_of_format)
            .append(errors_of_nullable)
            .append(errors_of_items)
            .append(errors_of_enum)
            .append(errors_all_of)
            .append(errors_one_of);

        output.to_result().map_err(Error::multiple)
    }
}

/// ex.1
/// ```yaml
/// type: object
/// required:
///   - id
/// properties:
///   id:
///     type: integer
///     format: int64
///   tag:
///     type: string
/// ```
///
/// ex.2
/// ```yaml
/// type: integer
/// format: int64
/// ```
///
/// ex.3
/// ```yaml
/// type: array
/// items:
///     type: string
/// ```
///
/// ex.4
/// ```yaml
/// type: string
/// enum:
///   - "value1"
///   - "value2"
/// ```
///
/// ex.5
/// ```yaml
/// allOf:
///   - $ref: '#/components/schemas/BasicErrorModel'
///   - type: object
///     required:
///       - rootCause
///     properties:
///       rootCause:
///         type: string
/// ```
///
/// ex.6
/// ```yaml
/// oneOf:
///   - $ref: '#/components/schemas/Cat'
///   - $ref: '#/components/schemas/Dog'
/// ```
///
/// rf. https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schema-object
#[derive(Clone, Debug)]
pub struct SchemaObject {
    pub title: Option<String>,

    pub description: Option<String>,

    /// > type - Value MUST be a string.
    /// > Multiple types via an array are not supported.
    pub data_type: Option<OpenApiDataType>,

    pub format: Option<FormatModifier>,

    /// rf. https://swagger.io/docs/specification/data-models/data-types/
    pub nullable: Option<bool>,

    pub required: Option<RequiredSchemaFields>,

    pub properties: Option<SchemaProperties>,

    pub items: Option<ArrayItems>,

    pub enum_values: Option<EnumValues>,

    pub all_of: Option<AllOf>,

    pub one_of: Option<OneOf>,
}

impl From<SchemaObject> for SchemaCase {
    fn from(this: SchemaObject) -> Self {
        Self::Schema(Box::new(this))
    }
}
