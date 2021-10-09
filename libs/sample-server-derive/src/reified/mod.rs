mod method_signature;
pub use method_signature::MethodSignature;

mod parameter;
pub use parameter::Parameter;

#[allow(unused)]
#[derive(Debug)]
enum Modifier {
    Pub,
    Async,
}

#[derive(Debug)]
struct MethodName(String);

#[derive(Debug)]
struct ReturnType(String);
