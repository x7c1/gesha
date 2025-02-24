#[cfg(test)]
mod invalid_specs {
    use gesha_core::conversions::Generator;
    use gesha_core::Error::OpenApiTypes;
    use gesha_rust_shapes::v3_0;
    use openapi_types::Error::UnknownDataType;

    #[test]
    fn has_unknown_type() {
        let converter = v3_0::DocumentConverter::default();
        let generator = Generator::new(&converter, "output-path-not-used");
        let result = generator.generate_from_file("./examples/v3_0/invalid/unknown-type.yaml");

        let Err(OpenApiTypes { cause, .. }) = result else {
            panic!("Err(OpenApiTypes) not returned: {:?}", result)
        };
        let traced_error = cause.trace_error().pop();
        let Some(err) = traced_error else {
            panic!("EnclosedErrors not returned: {:?}", traced_error)
        };
        let UnknownDataType { found } = err.cause else {
            panic!("UnknownDataType not returned: {:?}", err)
        };
        assert_eq!(err.keys, &["Foo"]);
        assert_eq!(found, "unknown-type");
    }
}
