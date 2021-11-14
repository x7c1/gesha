use Parameter::{Arg, RefSelf};

#[allow(unused)]
#[derive(Debug)]
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
            (Some(name), Some(type_name)) => Arg {
                name: name.join(""),
                type_name: type_name.join(""),
            },
            (Some(x), None) if x == ["&", "self"] => RefSelf,
            _ => panic!("unknown pattern: {:?}", strings),
        }
    }
}

impl PartialEq for Parameter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RefSelf, RefSelf) => true,
            (Arg { type_name: x1, .. }, Arg { type_name: x2, .. }) if x1 == x2 => true,
            _ => false,
        }
    }
}
