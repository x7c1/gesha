mod from_json {
    use examples_v3_0::components::schemas::ref_property::schemas::{
        AllOfSample, Container, EnumSample, NewTypeSample, ObjectSample,
    };

    #[test]
    fn ok() {
        let actual = serde_json::from_str::<Container>(
            r#"{
                "x0": {
                    "id": 0,
                    "name": "x0-name"
                },
                "x1": "x1-value",
                "x2": "a",
                "x3": {
                    "id": 3,
                    "name": "x3-name"
                }
            }"#,
        )
        .unwrap();
        let expected = Container {
            x0: ObjectSample {
                id: 0,
                name: "x0-name".to_string(),
            },
            x1: NewTypeSample::from("x1-value".to_string()),
            x2: EnumSample::A,
            x3: AllOfSample {
                id: 3,
                name: "x3-name".to_string(),
            },
        };
        assert_eq!(actual, expected)
    }
}

mod to_json {
    use crate::components::flatten;
    use examples_v3_0::components::schemas::ref_property::schemas::{
        AllOfSample, Container, EnumSample, NewTypeSample, ObjectSample,
    };

    #[test]
    fn ok() {
        let sample = Container {
            x0: ObjectSample {
                id: 0,
                name: "x0-name".to_string(),
            },
            x1: NewTypeSample::from("x1-value".to_string()),
            x2: EnumSample::A,
            x3: AllOfSample {
                id: 3,
                name: "x3-name".to_string(),
            },
        };
        let actual = serde_json::to_string(&sample).unwrap();
        let expected = flatten(
            r#"{
                "x0": {
                    "id": 0,
                    "name": "x0-name"
                },
                "x1": "x1-value",
                "x2": "a",
                "x3": {
                    "id": 3,
                    "name": "x3-name"
                }
            }"#,
        );
        assert_eq!(actual, expected)
    }
}
