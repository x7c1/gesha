mod to_json {
    use examples_v3_0::components::schemas::object_simple::schemas::Pet;

    #[test]
    fn ok() {
        let pet = Pet {
            id: 123,
            name: Some("sample_name".to_string()),
        };
        let actual = serde_json::to_string(&pet).unwrap();
        assert_eq!(actual, r#"{"id":123,"name":"sample_name"}"#)
    }

    #[test]
    fn ok_empty_field() {
        let pet = Pet {
            id: 123,
            name: None,
        };
        let actual = serde_json::to_string(&pet).unwrap();
        assert_eq!(actual, r#"{"id":123}"#)
    }
}

mod from_json {
    use examples_v3_0::components::schemas::object_simple::schemas::Pet;

    #[test]
    fn ok() {
        let actual = serde_json::from_str::<Pet>(r#"{"id":111,"name":"sample"}"#).unwrap();
        assert_eq!(
            actual,
            Pet {
                id: 111,
                name: Some("sample".to_string())
            }
        )
    }

    #[test]
    fn ok_empty_field() {
        let actual = serde_json::from_str::<Pet>(r#"{"id":111}"#).unwrap();
        assert_eq!(
            actual,
            Pet {
                id: 111,
                name: None,
            }
        )
    }
}
