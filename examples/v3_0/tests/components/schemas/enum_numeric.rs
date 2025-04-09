use examples_v3_0::components::schemas::enums_numeric::core::Error;
use examples_v3_0::components::schemas::enums_numeric::schemas::IntEnum;
use rstest::rstest;

#[rstest]
#[case(IntEnum::_0, 0)]
#[case(IntEnum::_100, 100)]
fn ok_into_i32(#[case] value: IntEnum, #[case] expected: i32) {
    let actual = i32::from(value);
    assert_eq!(actual, expected);
}

#[rstest]
#[case(0, IntEnum::_0)]
#[case(100, IntEnum::_100)]
fn ok_try_from_i32(#[case] value: i32, #[case] expected: IntEnum) {
    let actual = IntEnum::try_from(value).unwrap();
    assert_eq!(actual, expected);
}

#[rstest]
#[case(-1)]
#[case(123)]
fn err_try_from(#[case] value: i32) {
    let actual = IntEnum::try_from(value).unwrap_err();
    assert!(matches!(actual, Error::UnknownEnumValue { .. }));
}
