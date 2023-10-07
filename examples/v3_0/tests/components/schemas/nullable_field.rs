mod deserialize {
    use examples_v3_0::components::schemas::nullable_field::core::Patch;
    use examples_v3_0::components::schemas::nullable_field::schemas::{Sample1, Sample2, Sample5};

    #[test]
    fn value() {
        let actual = serde_json::from_str::<Sample1>(
            r#"
                { "x1": "foo" }
            "#,
        )
        .unwrap();
        let expected = Sample1 {
            x1: Patch::Value("foo".to_string()),
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn null() {
        let actual = serde_json::from_str::<Sample1>(
            r#"
                { "x1": null }
            "#,
        )
        .unwrap();
        let expected = Sample1 { x1: Patch::Null };
        assert_eq!(actual, expected)
    }

    #[test]
    fn absent() {
        let actual = serde_json::from_str::<Sample1>("{}").unwrap();
        let expected = Sample1 { x1: Patch::Absent };
        assert_eq!(actual, expected)
    }

    #[test]
    fn null_required() {
        let actual = serde_json::from_str::<Sample2>(
            r#"
                { "x1": null }
            "#,
        )
        .unwrap();
        let expected = Sample2 { x1: None };
        assert_eq!(actual, expected)
    }

    #[test]
    fn null_required_ref() {
        let actual = serde_json::from_str::<Sample5>(
            r#"
                { "pet1": null }
            "#,
        )
        .unwrap();
        let expected = Sample5 { pet1: None };
        assert_eq!(actual, expected)
    }
}

mod serialize {
    use crate::components::flatten;
    use examples_v3_0::components::schemas::nullable_field::core::Patch;
    use examples_v3_0::components::schemas::nullable_field::schemas::{Sample1, Sample2, Sample5};

    #[test]
    fn value() {
        let sample = Sample1 {
            x1: Patch::Value("foo".to_string()),
        };
        let actual = serde_json::to_string(&sample).unwrap();
        let expected = flatten(
            r#"{
                "x1": "foo"
            }"#,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn null() {
        let sample = Sample1 { x1: Patch::Null };
        let actual = serde_json::to_string(&sample).unwrap();
        let expected = flatten(
            r#"{
                "x1": null
            }"#,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn absent() {
        let sample = Sample1 { x1: Patch::Absent };
        let actual = serde_json::to_string(&sample).unwrap();
        let expected = "{}";
        assert_eq!(actual, expected);
    }

    #[test]
    fn null_required() {
        let sample = Sample2 { x1: None };
        let actual = serde_json::to_string(&sample).unwrap();
        let expected = flatten(
            r#"{
                "x1": null
            }"#,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn null_required_ref() {
        let sample = Sample5 { pet1: None };
        let actual = serde_json::to_string(&sample).unwrap();
        let expected = flatten(
            r#"{
                "pet1": null
            }"#,
        );
        assert_eq!(actual, expected);
    }
}
