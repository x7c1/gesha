use crate::reified::{MethodName, Modifier, Parameter, ReturnType};
use proc_macro2::{TokenStream, TokenTree};

#[allow(unused)]
use quote::quote;

#[derive(Debug)]
pub struct MethodSignature {
    modifiers: Vec<Modifier>,
    method_name: MethodName,
    parameters: Vec<Parameter>,
    return_type: ReturnType,
}

impl MethodSignature {
    #[allow(unused)]
    fn from_stream(stream: TokenStream) -> Self {
        let mut iter = stream.into_iter();
        let modifiers = extract_modifiers(&mut iter);
        let method_name = extract_method_name(&mut iter);
        let parameters = extract_parameters(&mut iter);

        dump_trees(&mut iter);

        MethodSignature {
            modifiers,
            method_name,
            parameters,
            return_type: ReturnType("hoge".to_string()),
        }
    }
}

#[allow(unused)]
fn extract_modifiers(iter: &mut impl Iterator<Item = TokenTree>) -> Vec<Modifier> {
    let mut modifiers: Vec<Modifier> = vec![];
    loop {
        match iter.next() {
            Some(TokenTree::Ident(ident)) => match ident.to_string().as_str() {
                "pub" => modifiers.push(Modifier::Pub),
                "async" => modifiers.push(Modifier::Async),
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

#[allow(unused)]
fn dump_trees(iter: &mut impl Iterator<Item = TokenTree>) {
    println!("[start] dump_trees");
    for tree in iter {
        println!("dump > tree: {:#?}", tree)
    }
}

#[test]
fn test_create_signature() {
    let stream = quote! {
        pub async fn index(&self, param1: u32, param2: foo::Bar) -> String
    };
    let signature = MethodSignature::from_stream(stream.into());
    println!("sig: {:#?}", signature)
}
