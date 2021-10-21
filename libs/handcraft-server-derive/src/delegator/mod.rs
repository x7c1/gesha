use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_delegate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let stream = quote! {
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
                let response = match handlers.show_pet_by_id(request).await {
                    inline::show_pet_by_id::Response::OK(body) => {
                        HttpResponse::Ok().json(body)
                    }
                    inline::show_pet_by_id::Response::InternalServerError(e) => {
                        HttpResponse::InternalServerError().json(e)
                    }
                };
                actix_web::Result::Ok(response)
            }
        }
    };
    stream.into()
}
