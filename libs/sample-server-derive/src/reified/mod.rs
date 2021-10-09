mod method_signature;
pub use method_signature::MethodSignature;

mod parameter;
pub use parameter::Parameter;

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum Modifier {
    Pub,
    Async,
}

#[derive(Debug, PartialEq)]
struct MethodName(String);

#[derive(Debug, PartialEq)]
struct ReturnType(String);
