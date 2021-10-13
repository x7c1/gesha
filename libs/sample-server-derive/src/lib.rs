mod extractor;
use extractor::validate_signature;

mod reified;

use proc_macro::TokenStream;
use quote::quote;

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
pub fn define(attr: TokenStream, item: TokenStream) -> TokenStream {
    let operation = attr.to_string();
    validate_signature(&operation, item.clone().into());
    item
}
