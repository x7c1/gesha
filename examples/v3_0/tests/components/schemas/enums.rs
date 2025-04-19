mod enum1 {
    use examples_v3_0::components::schemas::enums::schemas::StringEnum1;

    #[test]
    fn to_json_string() {
        let target = StringEnum1::Error1;
        let actual = serde_json::to_string(&target).unwrap();
        let expected = r#""ERROR1""#;
        assert_eq!(actual, expected)
    }

    #[test]
    fn from_string() {
        let source = "ERROR1".to_string();
        let actual = StringEnum1::try_from(source).unwrap();
        let expected = StringEnum1::Error1;
        assert_eq!(actual, expected)
    }

    #[test]
    fn from_str() {
        let source = "ERROR2";
        let actual = StringEnum1::try_from(source).unwrap();
        let expected = StringEnum1::Error2;
        assert_eq!(actual, expected)
    }

    #[test]
    fn into_string() {
        let actual: String = StringEnum1::Error1.into();
        let expected = "ERROR1".to_string();
        assert_eq!(actual, expected)
    }

    #[test]
    fn into_str() {
        let actual: &str = StringEnum1::Error2.into();
        let expected = "ERROR2";
        assert_eq!(actual, expected)
    }

    #[test]
    fn as_ref_str() {
        let actual: &str = StringEnum1::Error2.as_ref();
        let expected = "ERROR2";
        assert_eq!(actual, expected)
    }
}

mod enum2 {
    use examples_v3_0::components::schemas::enums::schemas::StringEnum2;

    #[test]
    fn from_json_string() {
        let actual = serde_json::from_str::<StringEnum2>(r#""ERROR_FOO""#).unwrap();
        let expected = StringEnum2::ErrorFoo;
        assert_eq!(actual, expected)
    }
}

mod enum4 {
    use examples_v3_0::components::schemas::enums::schemas::StringEnum4;

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
