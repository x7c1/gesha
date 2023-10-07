mod to_json {
    use crate::components::flatten;
    use examples_v3_0::components::schemas::object_inline::schemas::sample_pet::RegisteredProfile;
    use examples_v3_0::components::schemas::object_inline::schemas::SamplePet;

    #[test]
    fn ok() {
        let pet = SamplePet {
            id: 123,
            registered_profile: Some(RegisteredProfile {
                name: "foo".to_string(),
                int1: None,
            }),
        };
        let actual = serde_json::to_string(&pet).unwrap();

        let expected = flatten(
            r#"{
                "id":123,
                "registered_profile": {
                    "name": "foo"
                }
            }"#,
        );
        assert_eq!(actual, expected)
    }
}

mod from_json {
    use examples_v3_0::components::schemas::object_inline::schemas::sample_pet::RegisteredProfile;
    use examples_v3_0::components::schemas::object_inline::schemas::SamplePet;
    use pretty_assertions::assert_eq;

    #[test]
    fn ok() {
        let actual = serde_json::from_str::<SamplePet>(
            r#"{
                "id": 123,
                "registered_profile": {
                    "name": "bar",
                    "int1": 123
                }
            }"#,
        )
        .unwrap();

        let expected = SamplePet {
            id: 123,
            registered_profile: Some(RegisteredProfile {
                name: "bar".to_string(),
                int1: Some(123),
            }),
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn ok_empty() {
        let actual = serde_json::from_str::<SamplePet>(
            r#"{
                "id": 123
            }"#,
        )
        .unwrap();

        let expected = SamplePet {
            id: 123,
            registered_profile: None,
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn ok_null() {
        let actual = serde_json::from_str::<SamplePet>(
            r#"{
                "id": 123,
                "registered_profile": null
            }"#,
        )
        .unwrap();

        let expected = SamplePet {
            id: 123,
            registered_profile: None,
        };
        assert_eq!(actual, expected)
    }
}
