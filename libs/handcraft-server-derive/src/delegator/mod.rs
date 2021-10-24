use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_delegate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    quote! {
        pub mod generated {
            use super::#struct_name;
            use handcraft_models::inline;
            use handcraft_server::BadRequestHandler;
            use actix_web::get;
            use actix_web::HttpRequest;
            use actix_web::HttpResponse;
            use actix_web::Responder;
            use actix_web::Result;
            use actix_web::web;

            fn foo1<A: handcraft_server::BadRequestHandler>(handler: A) {
                println!("dummy");
            }

            fn foo2(handler: #struct_name) {
                foo1(handler);
            }

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
            ) -> Result<HttpResponse> {
                handcraft_server::list_pets::delegate(
                    raw,
                    |x| handlers.list_pets(x),
                    |x| handlers.on_bad_request(x),
                ).await
            }

            #[get("/pets/{pet_id}")]
            pub async fn show_pet_by_id(
                handlers: web::Data<#struct_name>,
                raw: HttpRequest,
            ) -> Result<HttpResponse> {
                handcraft_server::show_pet_by_id::delegate(
                    raw,
                    |x| handlers.show_pet_by_id(x),
                    |x| handlers.on_bad_request(x),
                ).await
            }
        }
    }
}
