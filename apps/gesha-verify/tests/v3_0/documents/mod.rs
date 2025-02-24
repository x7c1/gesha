#[cfg(test)]
mod invalid_specs {
    use gesha_core::conversions::Generator;
    use gesha_rust_shapes::v3_0;
    use pretty_assertions::assert_eq;

    #[test]
    fn unknown_type() {
        let converter = v3_0::DocumentConverter::default();
        let generator = Generator::new(&converter, "output-path-not-used");
        let result = generator.generate_from_file("./examples/v3_0/invalid/unknown-type.yaml");
        let actual = format!("{result:#?}");
        let expected = include_str!("unknown-type.txt");
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_errors() {
        let converter = v3_0::DocumentConverter::default();
        let generator = Generator::new(&converter, "output-path-not-used");
        let result = generator.generate_from_file("./examples/v3_0/invalid/multiple-errors.yaml");
        let actual = format!("{result:#?}");
        let expected = include_str!("multiple-errors.txt");
        assert_eq!(actual, expected);
    }
}
