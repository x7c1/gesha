use crate::v3_0::{FormatModifier, OpenApiDataType, ReferenceObject};
use indexmap::{IndexMap, IndexSet};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsObject
#[derive(Debug)]
pub struct ComponentsObject {
    pub schemas: Option<SchemasObject>,
}

pub type SchemasObject = IndexMap<SchemaFieldName, SchemaCase>;

/// > All the fixed fields declared above are objects
/// > that MUST use keys that match the regular expression: ^[a-zA-Z0-9\.\-_]+$.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct SchemaFieldName(String);

impl SchemaFieldName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        SchemaFieldName(a.into())
    }
}

impl From<SchemaFieldName> for String {
    fn from(this: SchemaFieldName) -> Self {
        this.0
    }
}

impl AsRef<str> for SchemaFieldName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Schema Object | Reference Object
#[derive(Debug)]
pub enum SchemaCase {
    Schema(SchemaObject),
    Reference(ReferenceObject),
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schemaObject
///
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
#[derive(Debug)]
pub struct SchemaObject {
    /// > type - Value MUST be a string.
    /// > Multiple types via an array are not supported.
    pub data_type: Option<OpenApiDataType>,

    pub format: Option<FormatModifier>,

    pub required: Option<RequiredSchemaFields>,

    pub properties: Option<SchemaProperties>,

    pub items: Option<ArrayItems>,
}

/// > properties - Property definitions MUST be a Schema Object
/// > and not a standard JSON Schema (inline or referenced).
///
/// see also: https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.16
pub type SchemaProperties = IndexMap<SchemaFieldName, SchemaCase>;

/// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.15
/// >The value of this keyword MUST be an array.  This array MUST have at
/// >least one element. Elements of this array MUST be strings, and MUST
/// >be unique.
#[derive(Debug)]
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
#[derive(Debug)]
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
