use crate::reified::MethodSignature;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use std::iter::FromIterator;

pub fn validate_signature(operation: &str, item: TokenStream) {
    let expected = to_expected_signature(operation);
    let actual = extract_actual_signature(item);

    // TODO: use simplified assertion, which can regard foo::Bar as Bar.
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
            async fn index(&self, id: u32, name: String) -> String
        },
        _ => panic!("unknown operation: {}", operation),
    };
    MethodSignature::from_stream(expected)
}

fn extract_actual_signature(item: TokenStream) -> MethodSignature {
    let actual = item.into_iter().take_while(|x| match x {
        TokenTree::Group(g) => match g.delimiter() {
            Delimiter::Brace => false,
            _ => true,
        },
        _ => true,
    });
    let stream = TokenStream::from_iter(actual);
    MethodSignature::from_stream(stream)
}
