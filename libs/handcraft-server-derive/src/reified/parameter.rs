#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum Parameter {
    /// e.g.
    /// &self
    RefSelf,
    /// e.g.
    /// foo: i32
    /// bar: Foo::Bar
    Arg { name: String, type_name: String },
}

impl Parameter {
    #[allow(unused)]
    pub fn new(strings: Vec<String>) -> Self {
        let mut pair = strings.splitn(2, |s| s == ":");
        match (pair.next(), pair.next()) {
            (Some(name), Some(type_name)) => Parameter::Arg {
                name: name.join(""),
                type_name: type_name.join(""),
            },
            (Some(x), None) if x == ["&", "self"] => Parameter::RefSelf,
            _ => panic!("unknown pattern: {:?}", strings),
        }
    }
}
