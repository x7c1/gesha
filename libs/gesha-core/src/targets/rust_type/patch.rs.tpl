#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Patch<A>(A);
