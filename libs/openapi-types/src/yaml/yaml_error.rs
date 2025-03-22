#[derive(Clone, Debug)]
pub enum YamlError {
    TypeMismatch { expected: String, found: String },
    UnknownType { found: String },
}
