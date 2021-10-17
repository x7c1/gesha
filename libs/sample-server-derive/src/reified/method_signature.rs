use crate::reified::{MethodName, Modifier, Parameter, ReturnType};
use proc_macro2::{TokenStream, TokenTree};

#[allow(unused)]
use quote::quote;
use Modifier::{Async, Pub};

#[derive(Debug, PartialEq)]
pub struct MethodSignature {
    modifiers: Vec<Modifier>,
    method_name: MethodName,
    parameters: Vec<Parameter>,
    return_type: ReturnType,
    rendered_output: String,
}

impl MethodSignature {
    #[allow(unused)]
    pub fn from_stream(stream: TokenStream) -> Self {
        let rendered_output = stream.to_string();
        let mut iter = stream.into_iter();
        let modifiers = extract_modifiers(&mut iter);
        let method_name = extract_method_name(&mut iter);
        let parameters = extract_parameters(&mut iter);
        let return_type = extract_return_type(&mut iter);

        MethodSignature {
            modifiers,
            method_name,
            parameters,
            return_type,
            rendered_output,
        }
    }
    pub fn method_name(&self) -> &str {
        &self.method_name.0
    }
    pub fn render(&self) -> &str {
        &self.rendered_output
    }
}

#[allow(unused)]
fn extract_modifiers(iter: &mut impl Iterator<Item = TokenTree>) -> Vec<Modifier> {
    let mut modifiers: Vec<Modifier> = vec![];
    loop {
        match iter.next() {
            Some(TokenTree::Ident(ident)) => match ident.to_string().as_str() {
                "pub" => modifiers.push(Pub),
                "async" => modifiers.push(Async),
                "fn" => break,
                unknown => panic!("unknown modifier found: {}", unknown),
            },
            _ => break,
        }
    }
    modifiers
}

#[allow(unused)]
fn extract_method_name(iter: &mut impl Iterator<Item = TokenTree>) -> MethodName {
    let name = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => panic!("method name not found"),
    };
    MethodName(name)
}

#[allow(unused)]
fn extract_parameters(iter: &mut impl Iterator<Item = TokenTree>) -> Vec<Parameter> {
    let stream = match iter.next() {
        Some(TokenTree::Group(group)) => group.stream(),
        _ => panic!("parameters not found"),
    };
    let mut parameters = vec![];
    let mut param_iter = stream.into_iter();
    let mut tokens: Vec<String> = vec![];
    loop {
        match param_iter.next() {
            Some(TokenTree::Punct(punct)) => match punct.to_string().as_str() {
                "," => {
                    let parameter = Parameter::new(tokens);
                    parameters.push(parameter);
                    tokens = vec![];
                }
                x => tokens.push(x.to_string()),
            },
            Some(x) => tokens.push(x.to_string()),
            None => break,
        }
    }
    if tokens.len() > 0 {
        parameters.push(Parameter::new(tokens));
    }
    parameters
}

fn extract_return_type(iter: &mut impl Iterator<Item = TokenTree>) -> ReturnType {
    match iter.next() {
        Some(TokenTree::Punct(punct)) => match punct.to_string().as_str() {
            "-" => (),
            x => panic!("unknown char: {}", x),
        },
        x => panic!("unexpected token: {:#?}", x),
    }
    match iter.next() {
        Some(TokenTree::Punct(punct)) => match punct.to_string().as_str() {
            ">" => (),
            x => panic!("unknown char: {}", x),
        },
        x => panic!("unexpected token: {:#?}", x),
    }
    let type_name = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        x => panic!("unexpected token: {:#?}", x),
    };
    ReturnType(Some(type_name))
}

#[allow(unused)]
fn dump_trees(iter: &mut impl Iterator<Item = TokenTree>) {
    println!("[start] dump_trees");
    for tree in iter {
        println!("dump > tree: {:#?}", tree)
    }
}

#[cfg(test)]
mod tests {
    use crate::reified::Modifier::{Async, Pub};
    use crate::reified::{MethodName, MethodSignature, Parameter, ReturnType};
    use pretty_assertions::assert_eq;
    use quote::quote;

    #[test]
    fn test_create_signature() {
        let stream = quote! {
            pub async fn index(&self, param1: u32, param2: foo::Bar) -> String
        };
        let actual = MethodSignature::from_stream(stream.into());
        let expected = MethodSignature {
            modifiers: vec![Pub, Async],
            method_name: MethodName("index".to_string()),
            parameters: vec![
                Parameter::RefSelf,
                Parameter::Arg {
                    name: "param1".to_string(),
                    type_name: "u32".to_string(),
                },
                Parameter::Arg {
                    name: "param2".to_string(),
                    type_name: "foo::Bar".to_string(),
                },
            ],
            return_type: ReturnType(Some("String".to_string())),
            rendered_output:
                "pub async fn index (& self , param1 : u32 , param2 : foo :: Bar) -> String"
                    .to_string(),
        };
        println!("sig: {:#?}", actual);
        assert_eq!(actual, expected);
    }
}
