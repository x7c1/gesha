mod to_json {
    use crate::components::flatten;
    use examples_v3_0::components::schemas::one_of::schemas::dog::Breed;
    use examples_v3_0::components::schemas::one_of::schemas::{Cat, Dog, Pet};

    #[test]
    fn ok() {
        let pet = Pet::Cat(Cat {
            hunts: Some(true),
            age: 3,
        });
        let actual = serde_json::to_string(&pet).unwrap();
        let expected = flatten(
            r#"{
                "hunts": true,
                "age": 3
            }"#,
        );
        assert_eq!(actual, expected);

        let pet = Pet::Dog(Dog {
            bark: Some(true),
            breed: Breed::Dingo,
        });
        let actual = serde_json::to_string(&pet).unwrap();
        let expected = flatten(
            r#"{
                "bark": true,
                "breed": "Dingo"
            }"#,
        );
        assert_eq!(actual, expected);
    }
}

mod from_json {
    use examples_v3_0::components::schemas::one_of::schemas::{Cat, Pet};

    #[test]
    fn ok() {
        let actual = serde_json::from_str::<Pet>(
            r#"{
                "hunts": true,
                "age": 3
            }"#,
        )
        .unwrap();

        let expected = Pet::Cat(Cat {
            hunts: Some(true),
            age: 3,
        });
        assert_eq!(actual, expected);
    }
}
