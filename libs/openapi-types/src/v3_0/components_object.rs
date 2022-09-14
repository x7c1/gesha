use crate::v3_0::ComponentName;
use crate::v3_0::{FormatModifier, OpenApiDataType, ReferenceObject, RequestBodiesObject};
use indexmap::{IndexMap, IndexSet};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsObject
#[derive(Debug)]
pub struct ComponentsObject {
    pub request_bodies: Option<RequestBodiesObject>,
    pub schemas: Option<SchemasObject>,
}

pub type SchemasObject = IndexMap<ComponentName, SchemaCase>;

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
/// rf. https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schemaObject
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
}

/// > properties - Property definitions MUST be a Schema Object
/// > and not a standard JSON Schema (inline or referenced).
///
/// see also: https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.16
pub type SchemaProperties = IndexMap<ComponentName, SchemaCase>;

/// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.15
/// >The value of this keyword MUST be an array.  This array MUST have at
/// >least one element. Elements of this array MUST be strings, and MUST
/// >be unique.
#[derive(Clone, Debug)]
pub struct RequiredSchemaFields(IndexSet<String>);

impl RequiredSchemaFields {
    pub fn new(fields: IndexSet<String>) -> Self {
        // TODO: check fields length
        Self(fields)
    }
    pub fn contains(&self, field_name: &str) -> bool {
        self.0.contains(field_name)
    }
}

/// > Value MUST be an object and not an array.
/// > Inline or referenced schema MUST be of a Schema Object and
/// > not a standard JSON Schema. items MUST be present if the type is array.
///
/// ---
///
/// see also: https://swagger.io/docs/specification/data-models/data-types/
///
#[derive(Clone, Debug)]
pub struct ArrayItems(Box<SchemaCase>);

impl ArrayItems {
    pub fn new(case: SchemaCase) -> Self {
        Self(Box::new(case))
    }
}

impl From<ArrayItems> for SchemaCase {
    fn from(xs: ArrayItems) -> Self {
        *xs.0
    }
}

/// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.20
/// > The value of this keyword MUST be an array. This array SHOULD have
/// > at least one element.  Elements in the array SHOULD be unique.
/// > Elements in the array MAY be of any type, including null.
pub type EnumValues = IndexSet<String>;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schemaObject
/// > Inline or referenced schema MUST be of a Schema Object and not a standard JSON Schema.
///
/// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.22
/// > This keyword's value MUST be an array.  This array MUST have at least one element.
///
/// > Elements of the array MUST be objects.  Each object MUST be a valid JSON Schema.
pub type AllOf = Vec<SchemaCase>;
