use crate::components::flatten;
use examples_v3_0::components::request_bodies::schema_ref::request_bodies::PetBody;
use examples_v3_0::components::request_bodies::schema_ref::schemas::Pet;

#[test]
fn to_json() {
    let pet = Pet { id: 123 };
    let pet_body = PetBody::ApplicationJson(pet);
    let actual = serde_json::to_string(&pet_body).unwrap();

    let expected = flatten(
        r#"{
            "id": 123
        }"#,
    );
    assert_eq!(actual, expected)
}

#[test]
fn from_json() {
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
