use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_delegate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    quote! {
        pub mod generated {
            use super::#struct_name;
            use handcraft_models::inline;
            use actix_web::get;
            use actix_web::HttpRequest;
            use actix_web::HttpResponse;
            use actix_web::Responder;
            use actix_web::Result;
            use actix_web::web;

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

            #[get("/pets")]
            pub async fn list_pets(
                handlers: web::Data<#struct_name>,
                raw: HttpRequest,
                query: web::Query<inline::list_pets::Query>,
            ) -> Result<HttpResponse> {
                let request = inline::list_pets::Request {
                    query: query.into_inner(),
                    raw,
                };
                let response = handlers.list_pets(request).await;
                let raw_response = inline::list_pets::Responder::to_raw(response);
                actix_web::Result::Ok(raw_response)
            }

            #[get("/pets/{pet_id}")]
            pub async fn show_pet_by_id(
                handlers: web::Data<#struct_name>,
                raw: HttpRequest,
                path: web::Path<inline::show_pet_by_id::Path>,
            ) -> Result<HttpResponse> {
                let request = inline::show_pet_by_id::Request {
                    path: path.into_inner(),
                    raw,
                };
                let response = handlers.show_pet_by_id(request).await;
                let raw_response = inline::show_pet_by_id::Responder::to_raw(response);
                actix_web::Result::Ok(raw_response)
            }
        }
    }
}
