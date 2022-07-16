mod camel_case_fields;
mod newtype;
mod nullable_field;
mod reserved_keywords;
mod struct_simple;

fn flatten(x: &str) -> String {
    x.replace(&[' ', '\n'], "")
}
