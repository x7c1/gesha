#[derive(Clone, Debug)]
pub enum YamlError {
    FieldNotExist { field: String },
    TypeMismatch { expected: String, found: String },
    UnknownType { found: String },
}
