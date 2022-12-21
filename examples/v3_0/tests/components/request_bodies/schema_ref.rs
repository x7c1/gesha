mod to_json {
    use crate::components::flatten;
    use examples_v3_0::components::request_bodies::schema_ref::core::MediaType;
    use examples_v3_0::components::request_bodies::schema_ref::request_bodies::PetBody;
    use examples_v3_0::components::request_bodies::schema_ref::schemas::Pet;

    #[test]
    fn ok() {
        let pet = Pet { id: 123 };
        let pet_body = PetBody::ApplicationJson(pet);
        assert_eq!(pet_body.media_type(), MediaType::ApplicationJson);

        let actual = serde_json::to_string(&pet_body).unwrap();

        let expected = flatten(
            r#"{
                "id": 123
            }"#,
        );
        assert_eq!(actual, expected)
    }
}

mod from_json {
    use examples_v3_0::components::request_bodies::schema_ref::core::Error;
    use examples_v3_0::components::request_bodies::schema_ref::request_bodies::PetBody;
    use examples_v3_0::components::request_bodies::schema_ref::schemas::Pet;

    #[test]
    fn ok() {
        let actual = PetBody::new(
            r#"{
                "id": 123
            }"#,
            "application/json",
        )
        .unwrap();

        let expected = PetBody::ApplicationJson(Pet { id: 123 });
        assert_eq!(actual, expected)
    }

    #[test]
    fn invalid_json() {
        let actual = PetBody::new(
            r#"{
                "abcde": 123
            }"#,
            "application/json",
        )
        .unwrap_err();

        assert!(matches!(actual, Error::InvalidJson(..)));
    }

    #[test]
    fn unsupported_media_type() {
        let actual = PetBody::new(
            r#"{
                "id": 123
            }"#,
            "abcde",
        )
        .unwrap_err();

        match actual {
            Error::UnsupportedMediaType { given } => {
                assert_eq!(given, "abcde")
            }
            _ => assert!(false),
        }
    }
}

mod media_type {
    use examples_v3_0::components::request_bodies::schema_ref::core::MediaType;

    #[test]
    fn display_json() {
        let actual = format!("{}", MediaType::ApplicationJson);
        let expected = "application/json";
        assert_eq!(actual, expected)
    }
}
