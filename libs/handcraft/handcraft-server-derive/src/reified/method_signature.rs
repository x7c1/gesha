use crate::reified::{MethodName, Modifier, Parameter, ReturnType};
use proc_macro2::{TokenStream, TokenTree};
use Modifier::{Async, Pub};
use TokenTree::{Ident, Punct};

#[derive(Debug)]
pub struct MethodSignature {
    modifiers: Vec<Modifier>,
    method_name: MethodName,
    parameters: Vec<Parameter>,
    return_type: ReturnType,
    rendered_output: String,
}

impl PartialEq for MethodSignature {
    fn eq(&self, other: &Self) -> bool {
        self.modifiers == other.modifiers
            && self.method_name == other.method_name
            && self.parameters == other.parameters
            && self.return_type == other.return_type
    }
}

impl MethodSignature {
    pub fn from_stream(stream: TokenStream) -> Self {
        let rendered_output = stream.to_string().replace("\n", " ");
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

fn extract_modifiers(iter: &mut impl Iterator<Item = TokenTree>) -> Vec<Modifier> {
    let mut modifiers: Vec<Modifier> = vec![];
    while let Some(Ident(ident)) = iter.next() {
        match ident.to_string().as_str() {
            "pub" => modifiers.push(Pub),
            "async" => modifiers.push(Async),
            "fn" => break,
            unknown => panic!("unknown modifier found: {}", unknown),
        }
    }
    modifiers
}

fn extract_method_name(iter: &mut impl Iterator<Item = TokenTree>) -> MethodName {
    let name = match iter.next() {
        Some(Ident(ident)) => ident.to_string(),
        _ => panic!("method name not found"),
    };
    MethodName(name)
}

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
            Some(Punct(punct)) => match punct.to_string().as_str() {
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
    if !tokens.is_empty() {
        parameters.push(Parameter::new(tokens));
    }
    parameters
}

fn extract_return_type(iter: &mut impl Iterator<Item = TokenTree>) -> ReturnType {
    match iter.next() {
        Some(Punct(punct)) => match punct.to_string().as_str() {
            "-" => (),
            x => panic!("unknown punct: {}", x),
        },
        Some(x) => panic!("unknown token: {}", x),
        None => {
            return ReturnType(None);
        }
    }
    match iter.next() {
        Some(Punct(punct)) => match punct.to_string().as_str() {
            ">" => (),
            x => panic!("unknown punct: {}", x),
        },
        x => panic!("unexpected token (expected [>]): {:#?}", x),
    }
    let type_name = iter
        .map(|tree| match tree {
            Ident(ident) => ident.to_string() + if ident == "impl" { " " } else { "" },
            Punct(punct) => punct.to_string(),
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("");

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
    fn test_signature() {
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
        // println!("actual: {:#?}", actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_signature_impl() {
        let stream = quote! {
            pub async fn show_pet_by_id(&self, req: show_pet_by_id::Request) -> impl show_pet_by_id::Responder
        };
        let actual = MethodSignature::from_stream(stream.into());
        let expected = MethodSignature {
            modifiers: vec![Pub, Async],
            method_name: MethodName("show_pet_by_id".to_string()),
            parameters: vec![
                Parameter::RefSelf,
                Parameter::Arg {
                    name: "req".to_string(),
                    type_name: "show_pet_by_id::Request".to_string(),
                },
            ],
            return_type: ReturnType(Some("impl show_pet_by_id::Responder".to_string())),
            rendered_output:
                "pub async fn show_pet_by_id (& self , req : show_pet_by_id :: Request) -> impl show_pet_by_id :: Responder"
                    .to_string(),
        };
        // println!("actual: {:#?}", actual);
        assert_eq!(actual, expected);
    }
}
