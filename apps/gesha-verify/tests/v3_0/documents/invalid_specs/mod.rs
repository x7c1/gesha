use gesha_core::Result;
use gesha_core::conversions::format_errors;
use gesha_core::io::Reader;
use gesha_rust_shapes::v3_0;
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case::unknown_type(
    "./examples/v3_0/invalid/unknown-type.yaml",
    include_str!("output/unknown-type.txt"))
]
#[case::multiple_errors(
    "./examples/v3_0/invalid/multiple-errors.yaml",
    include_str!("output/multiple-errors.txt"))
]
#[case::type_unspecified(
    "./examples/v3_0/invalid/type-unspecified.yaml",
    include_str!("output/type-unspecified.txt"))
]
#[case::empty_required(
    "./examples/v3_0/invalid/empty-required.yaml",
    include_str!("output/empty-required.txt"))
]
#[case::invalid_type(
    "./examples/v3_0/invalid/invalid-type.yaml",
    include_str!("output/invalid-type.txt"))
]
fn show_errors(#[case] schema: &str, #[case] expected: &str) -> Result<()> {
    let converter = v3_0::DocumentConverter::default();
    let output = Reader::new(schema).open_target_type(&converter)?;
    let actual = format_errors(output).unwrap();
    assert_eq!(actual, expected);
    Ok(())
}
