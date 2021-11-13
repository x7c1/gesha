use crate::reified::MethodSignature;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use std::iter::FromIterator;

pub fn validate_signature(item: TokenStream) {
    let actual = extract_actual_signature(item);
    let operation = actual.method_name();
    let expected = to_expected_signature(operation);
    if expected != actual {
        panic!(
            "inconsistent signature:\nexpected: {}\n  actual: {}",
            expected.render(),
            actual.render(),
        )
    }
}

fn to_expected_signature(operation: &str) -> MethodSignature {
    let expected = match operation {
        "index" => quote! {
            pub async fn index(&self, req: index::Request) -> String
        },
        "create_pets" => quote! {
            pub async fn create_pets(&self, req: create_pets::Request) -> impl create_pets::Responder
        },
        "find_pets" => quote! {
            pub async fn find_pets(&self, req: find_pets::Request) -> impl find_pets::Responder
        },
        "show_pet_by_id" => quote! {
            pub async fn show_pet_by_id(&self, req: show_pet_by_id::Request) -> impl show_pet_by_id::Responder
        },
        "list_pets" => quote! {
            pub async fn list_pets(&self, req: list_pets::Request) -> impl list_pets::Responder
        },
        _ => panic!("unknown operation: {}", operation),
    };
    MethodSignature::from_stream(expected)
}

fn extract_actual_signature(item: TokenStream) -> MethodSignature {
    let actual = item.into_iter().take_while(|x| match x {
        TokenTree::Group(g) => !matches!(g.delimiter(), Delimiter::Brace),
        _ => true,
    });
    let stream = TokenStream::from_iter(actual);
    MethodSignature::from_stream(stream)
}
