pub const COMPONENTS_PATH: &str = "examples/v3_0/src/components";

pub fn schemas_files() -> Vec<&'static str> {
    vec![
        "object_simple.yaml",
        "numeric_fields.yaml",
        "boolean_field.yaml",
        "array.yaml",
        "ref_property.yaml",
        "ref_items.yaml",
        "optional_field.yaml",
        "newtype.yaml",
        "newtype_numeric.yaml",
        "reserved_keywords.yaml",
        "enums.yaml",
        "all_of.yaml",
        "all_of_ref.yaml",
        "camel_case_fields.yaml",
        "title_description.yaml",
        "nullable_field.yaml",
        "object_inline.yaml",
        "object_inline_nested.yaml",
        "object_inline_ref.yaml",
        "object_inline_all_of.yaml",
        "object_inline_nullable.yaml",
        "all_of_inline_all_of.yaml",
        "all_of_override_optional.yaml",
        "object_inline_enum.yaml",
        "one_of.yaml",
        "object_inline_one_of.yaml",
    ]
}

pub fn request_bodies_files() -> Vec<&'static str> {
    vec!["schema_ref.yaml"]
}