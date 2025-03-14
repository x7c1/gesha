use examples_v3_0::components::schemas::enums_mixed_type::schemas::MixedTypeEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Example1 {
    pub x1: MixedTypeEnum,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Example2 {
    pub values: Vec<MixedTypeEnum>,
}

mod to_json {
    use super::Example1;
    use crate::components::flatten;
    use examples_v3_0::components::schemas::enums_mixed_type::schemas::MixedTypeEnum;
    use pretty_assertions::assert_eq;

    #[test]
    fn ok_1() {
        let target = Example1 {
            x1: MixedTypeEnum::_2000,
        };
        let actual = serde_json::to_string(&target).unwrap();
        let expected = flatten(
            r#"
            {
                "x1": "2000"
            }
            "#,
        );
        assert_eq!(actual, expected)
    }

    #[test]
    fn ok_2() {
        let target = Example1 {
            x1: MixedTypeEnum::_1000,
        };
        let actual = serde_json::to_string(&target).unwrap();
        let expected = flatten(
            r#"
            {
                "x1": 1000
            }
            "#,
        );
        assert_eq!(actual, expected)
    }
}

mod from_json {
    use super::Example1;
    use examples_v3_0::components::schemas::enums_mixed_type::schemas::MixedTypeEnum;
    use pretty_assertions::assert_eq;

    #[test]
    fn ok_1() {
        let actual = serde_json::from_str::<Example1>(
            r#"
            {
                "x1": 1000
            }
            "#,
        )
        .unwrap();

        let expected = Example1 {
            x1: MixedTypeEnum::_1000,
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn ok_2() {
        let actual = serde_json::from_str::<Example1>(
            r#"
            {
                "x1": "2001"
            }"#,
        )
        .unwrap();

        let expected = Example1 {
            x1: MixedTypeEnum::_2001,
        };
        assert_eq!(actual, expected)
    }
}
