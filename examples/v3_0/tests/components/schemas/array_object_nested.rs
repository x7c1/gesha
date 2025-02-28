mod to_json {
    use crate::components::flatten;
    use examples_v3_0::components::schemas::array_object_nested::schemas::number_list::NumberListItemItem;
    use examples_v3_0::components::schemas::array_object_nested::schemas::NumberList;
    use pretty_assertions::assert_eq;

    #[test]
    fn ok() {
        let list = NumberList::from(vec![
            vec![
                NumberListItemItem { a1: Some(123.0) },
                NumberListItemItem { a1: Some(456.0) },
            ],
            vec![],
        ]);
        let actual = serde_json::to_string(&list).unwrap();
        let expected = flatten(
            r#"[
                [
                    {"a1": 123.0},
                    {"a1": 456.0}
                ],
                []
            ]"#,
        );
        assert_eq!(actual, expected)
    }
}

mod from_json {
    use examples_v3_0::components::schemas::array_object_nested::schemas::number_list::NumberListItemItem;
    use examples_v3_0::components::schemas::array_object_nested::schemas::NumberList;
    use pretty_assertions::assert_eq;

    #[test]
    fn ok() {
        let actual = serde_json::from_str::<NumberList>(
            r#"[
                [
                    {"a1": 123.0},
                    {"a1": 456.0}
                ],
                []
            ]"#,
        )
        .unwrap();

        let expected = NumberList::from(vec![
            vec![
                NumberListItemItem { a1: Some(123.0) },
                NumberListItemItem { a1: Some(456.0) },
            ],
            vec![],
        ]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn ok_empty() {
        let actual = serde_json::from_str::<NumberList>(r#"[]"#).unwrap();
        let expected = NumberList::from(vec![]);
        assert_eq!(actual, expected)
    }
}
