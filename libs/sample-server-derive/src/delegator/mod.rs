use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_delegate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let stream = quote! {
        mod generated {
            use super::#struct_name;
            use actix_web::web;
            use actix_web::get;
            use actix_web::Responder;
            use actix_web::HttpRequest;
            use sample_models::inline;

            #[get("/{id}/{name}/index.html")]
            pub async fn index(
                handlers: web::Data<#struct_name>,
                raw: HttpRequest,
                path: web::Path<inline::index::Path>,
            ) -> impl Responder {
                let request = inline::index::Request {
                    path: path.into_inner(),
                    raw,
                };
                handlers.index(request).await
            }
        }
    };
    stream.into()
}
