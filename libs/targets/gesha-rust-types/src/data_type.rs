use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum DataType {
    Bool,
    Int32,
    Int64,
    Float32,
    Float64,
    Option {
        data_type: Box<DataType>,
        nullable: bool,
    },
    Patch(Box<DataType>),
    String,
    Vec(Box<DataType>),
    Custom(String),
}

impl DataType {
    pub fn is_non_nullable_option(&self) -> bool {
        matches!(
            self,
            DataType::Option {
                nullable: false,
                ..
            }
        )
    }
    pub fn is_patch(&self) -> bool {
        matches!(self, DataType::Patch(_))
    }
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
            DataType::Option { data_type, .. } => format!("Option<{}>", String::from(*data_type)),
            DataType::Patch(x) => format!("Patch<{}>", String::from(*x)),
            DataType::String => "String".to_string(),
            DataType::Vec(x) => format!("Vec<{}>", String::from(*x)),
            DataType::Custom(x) => x,
        }
    }
}
