use crate::v3_0::all_of::AllOf;
use crate::v3_0::array_items::ArrayItems;
use crate::v3_0::{
    EnumValues, FormatModifier, OneOf, OpenApiDataType, ReferenceObject, RequiredSchemaFields,
    SchemaProperties,
};

/// Schema Object | Reference Object
#[derive(Clone, Debug)]
pub enum SchemaCase {
    Schema(Box<SchemaObject>),
    Reference(ReferenceObject<SchemaObject>),
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
