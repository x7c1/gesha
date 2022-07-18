mod enum1 {
    use examples_v3_0::components::enums::schemas::StringEnum1;

    #[test]
    fn to_json_string() {
        let target = StringEnum1::Error1;
        let actual = serde_json::to_string(&target).unwrap();
        let expected = r#""ERROR1""#;
        assert_eq!(actual, expected)
    }
}

mod enum2 {
    use examples_v3_0::components::enums::schemas::StringEnum2;

    #[test]
    fn from_json_string() {
        let actual = serde_json::from_str::<StringEnum2>(r#""ERROR_FOO""#).unwrap();
        let expected = StringEnum2::ErrorFoo;
        assert_eq!(actual, expected)
    }
}

mod enum4 {
    use examples_v3_0::components::enums::schemas::StringEnum4;

    #[test]
    fn to_json_string() {
        let target = StringEnum4::Foo;
        let actual = serde_json::to_string(&target).unwrap();
        let expected = r#""Foo""#;
        assert_eq!(actual, expected)
    }

    #[test]
    fn from_json_string() {
        let actual = serde_json::from_str::<StringEnum4>(r#""Bar""#).unwrap();
        let expected = StringEnum4::Bar;
        assert_eq!(actual, expected)
    }
}
