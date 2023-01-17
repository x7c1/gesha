mod all_of_item_shape;
pub use all_of_item_shape::AllOfItemShape;

mod all_of_shape;
pub use all_of_shape::AllOfShape;

mod definition_shape;
pub use definition_shape::DefinitionShape;

mod field_shape;
pub use field_shape::FieldShape;

mod mod_shape;
pub use mod_shape::ModShape;

mod one_of_shape;
pub use one_of_shape::OneOfShape;

mod optionality;
pub use optionality::Optionality;

mod schemas_shape;
pub use schemas_shape::SchemasShape;

mod struct_shape;
pub use struct_shape::StructShape;

mod type_header_shape;
pub use type_header_shape::TypeHeaderShape;

mod type_path;
pub use type_path::TypePath;

mod type_shape;
pub use type_shape::TypeShape;

use openapi_types::v3_0::{ReferenceObject, SchemaObject};
type Ref = ReferenceObject<SchemaObject>;
