mod reified;

use proc_macro::TokenTree;
use proc_macro::{Delimiter, TokenStream};
use quote::quote;
use std::iter::FromIterator;

#[proc_macro_derive(Sample)]
pub fn delegate_api_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_delegate_macro(&ast)
}

fn impl_delegate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let gen = quote! {
        use actix_web::web;
        use actix_web::get;
        use actix_web::Responder;

        #[get("/{id}/{name}/index.html")]
        pub async fn index(
            api: web::Data<#struct_name>,
            web::Path((id, name)): web::Path<(u32, String)>,
        ) -> impl Responder {
            api.index(id, name).await
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn delegate_to(attr: TokenStream, item: TokenStream) -> TokenStream {
    let operation = attr.to_string();
    let expected = to_expected_signature(operation.as_str());
    let actual = extract_actual_signature(item.clone());
    validate_signature(actual, expected);
    item
}

fn to_expected_signature(operation: &str) -> TokenStream {
    let expected = match operation {
        "index" => quote! {
            async fn index(&self, id: u32, name: String) -> String
        },
        _ => panic!("unknown operation: {}", operation),
    };
    expected.into()
}

fn extract_actual_signature(item: TokenStream) -> TokenStream {
    let actual_signature = item.into_iter().take_while(|x| match x {
        TokenTree::Group(g) => match g.delimiter() {
            Delimiter::Brace => false,
            _ => true,
        },
        _ => true,
    });
    TokenStream::from_iter(actual_signature)
}

fn validate_signature(actual: TokenStream, expected: TokenStream) {
    println!("actual:{}", actual.to_string());
    println!("expected:{}", expected.to_string());

    let mut actual = actual.into_iter();

    for x0 in expected {
        match x0 {
            TokenTree::Group(_) => {}
            TokenTree::Ident(_) => {}
            TokenTree::Punct(_) => {}
            TokenTree::Literal(_) => {}
        }
        let _y0 = actual.next().unwrap();
        // println!("actual:{:#?}", y0);
        // println!("expected:{:#?}", x0);
    }
}
