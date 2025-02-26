#[cfg(test)]
mod invalid_specs {
    use gesha_core::conversions::format_errors;
    use gesha_core::io::Reader;
    use gesha_core::Result;
    use gesha_rust_shapes::v3_0;
    use pretty_assertions::assert_eq;

    #[test]
    fn unknown_type() -> Result<()> {
        let converter = v3_0::DocumentConverter::default();
        let schema = "./examples/v3_0/invalid/unknown-type.yaml";
        let output = Reader::new(schema).open_target_type(&converter)?;

        let actual = format_errors(output).unwrap();
        let expected = include_str!("unknown-type.txt");

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn multiple_errors() -> Result<()> {
        let converter = v3_0::DocumentConverter::default();
        let schema = "./examples/v3_0/invalid/multiple-errors.yaml";
        let output = Reader::new(schema).open_target_type(&converter)?;

        let actual = format_errors(output).unwrap();
        let expected = include_str!("multiple-errors.txt");

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn type_unspecified() -> Result<()> {
        let converter = v3_0::DocumentConverter::default();
        let schema = "./examples/v3_0/invalid/type-unspecified.yaml";
        let output = Reader::new(schema).open_target_type(&converter)?;

        let actual = format_errors(output).unwrap();
        let expected = include_str!("type-unspecified.txt");

        assert_eq!(actual, expected);
        Ok(())
    }
}
