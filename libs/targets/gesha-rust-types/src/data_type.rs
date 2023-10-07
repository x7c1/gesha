use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum DataType {
    Bool,
    Int32,
    Int64,
    Float32,
    Float64,
    Option(Box<DataType>),
    Patch(Box<DataType>),
    String,
    Vec(Box<DataType>),
    Custom(String),
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = String::from(self.clone());
        Display::fmt(&x, f)
    }
}

impl From<DataType> for String {
    fn from(x: DataType) -> Self {
        match x {
            DataType::Bool => "bool".to_string(),
            DataType::Int32 => "i32".to_string(),
            DataType::Int64 => "i64".to_string(),
            DataType::Float32 => "f32".to_string(),
            DataType::Float64 => "f64".to_string(),
            DataType::Option(x) => format!("Option<{}>", String::from(*x)),
            DataType::Patch(x) => format!("Patch<{}>", String::from(*x)),
            DataType::String => "String".to_string(),
            DataType::Vec(x) => format!("Vec<{}>", String::from(*x)),
            DataType::Custom(x) => x,
        }
    }
}
