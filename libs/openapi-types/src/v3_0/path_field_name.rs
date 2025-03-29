use crate::Result;
use crate::v3_0::SpecViolation::InvalidPathFieldName;

/// e.g. /pets
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PathFieldName(String);

impl PathFieldName {
    /// > The field name MUST begin with a forward slash (/).
    pub fn new<A: Into<String>>(a: A) -> Result<Self> {
        let field = a.into();
        if !field.starts_with("/") {
            return Err(InvalidPathFieldName { field })?;
        }
        Ok(Self(field))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn ok_field_name() {
        let field = PathFieldName::new("/pets").unwrap();
        assert_eq!(field.0, "/pets");
    }

    #[test]
    fn err_field_name() {
        let err = match PathFieldName::new("pets").unwrap_err() {
            Error::SpecViolation(e) => e,
            other => panic!("Expected SpecViolation, got {:?}", other),
        };
        let expected = InvalidPathFieldName {
            field: "pets".to_string(),
        };
        assert_eq!(err, expected.into());
    }
}
