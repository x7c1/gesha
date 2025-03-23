use examples_v3_0::components::schemas::enums_mixed_type::schemas::MixedTypeEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Example1 {
    pub x0: MixedTypeEnum,
    pub x1: MixedTypeEnum,
    pub x2: MixedTypeEnum,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Example2 {
    pub values: Vec<MixedTypeEnum>,
}

mod to_json {
    use super::{Example1, Example2};
    use crate::components::flatten;
    use examples_v3_0::components::schemas::enums_mixed_type::schemas::MixedTypeEnum;
    use pretty_assertions::assert_eq;

    #[test]
    fn ok_1() {
        let target = Example1 {
            x0: MixedTypeEnum::True,
            x1: MixedTypeEnum::_2000,
            x2: MixedTypeEnum::Minus42,
        };
        let actual = serde_json::to_string(&target).unwrap();
        let expected = flatten(
            r#"
            {
                "x0": true,
                "x1": "2000",
                "x2": -42
            }
            "#,
        );
        assert_eq!(actual, expected)
    }

    #[test]
    fn ok_2() {
        let target = Example1 {
            x0: MixedTypeEnum::False,
            x1: MixedTypeEnum::_1000,
            x2: MixedTypeEnum::_2002,
        };
        let actual = serde_json::to_string(&target).unwrap();
        let expected = flatten(
            r#"
            {
                "x0": false,
                "x1": 1000,
                "x2": "2002"
            }
            "#,
        );
        assert_eq!(actual, expected)
    }

    #[test]
    fn ok_3() {
        let target = Example2 {
            values: vec![
                MixedTypeEnum::_1000,
                MixedTypeEnum::_2000,
                MixedTypeEnum::Minus42,
            ],
        };
        let actual = serde_json::to_string(&target).unwrap();
        let expected = flatten(
            r#"
            {
                "values": [
                    1000,
                    "2000",
                    -42
                ]
            }
            "#,
        );
        assert_eq!(actual, expected)
    }
}

mod from_json {
    use super::{Example1, Example2};
    use examples_v3_0::components::schemas::enums_mixed_type::schemas::MixedTypeEnum;
    use pretty_assertions::assert_eq;

    #[test]
    fn ok_1() {
        let actual = serde_json::from_str::<Example1>(
            r#"
            {
                "x0": true,
                "x1": "2000",
                "x2": -42
            }
            "#,
        )
        .unwrap();

        let expected = Example1 {
            x0: MixedTypeEnum::True,
            x1: MixedTypeEnum::_2000,
            x2: MixedTypeEnum::Minus42,
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn ok_2() {
        let actual = serde_json::from_str::<Example1>(
            r#"
            {
                "x0": false,
                "x1": 1000,
                "x2": "2002"
            }"#,
        )
        .unwrap();

        let expected = Example1 {
            x0: MixedTypeEnum::False,
            x1: MixedTypeEnum::_1000,
            x2: MixedTypeEnum::_2002,
        };
        assert_eq!(actual, expected)
    }
    #[test]
    fn ok_3() {
        let actual = serde_json::from_str::<Example2>(
            r#"
            {
                "values": [
                    true,
                    1000,
                    "2000",
                    -42
                ]
            }
            "#,
        )
        .unwrap();

        let expected = Example2 {
            values: vec![
                MixedTypeEnum::True,
                MixedTypeEnum::_1000,
                MixedTypeEnum::_2000,
                MixedTypeEnum::Minus42,
            ],
        };
        assert_eq!(actual, expected)
    }
}
