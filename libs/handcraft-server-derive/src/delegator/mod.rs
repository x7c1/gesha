use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn impl_delegate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let define_create_pets = define_service(struct_name, "create_pets");
    let define_find_pets = define_service(struct_name, "find_pets");
    let define_list_pets = define_service(struct_name, "list_pets");
    let define_show_pet_by_id = define_service(struct_name, "show_pet_by_id");

    quote! {
        pub mod generated {
            use super::#struct_name;
            use handcraft_models::inline;
            use handcraft_server::BadRequestHandler;
            use actix_web::{get, post};
            use actix_web::HttpRequest;
            use actix_web::HttpResponse;
            use actix_web::Responder;
            use actix_web::Result;
            use actix_web::web;

            // dummy function to check whether handler implements BadRequestHandler or not.
            fn foo1<A: handcraft_server::BadRequestHandler>(handler: A) {
                println!("dummy");
            }

            // dummy function defined to call foo1 above.
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
            #define_list_pets

            #[post("/pets")]
            #define_create_pets

            #[get("/pets/{pet_id}")]
            #define_show_pet_by_id

            #[get("/petstore-expanded/pets")]
            #define_find_pets
        }
    }
}

fn define_service(struct_name: &Ident, operation: &str) -> TokenStream {
    let op = format_ident!("{}", operation);
    quote! {
        pub async fn #op(
            handlers: web::Data<#struct_name>,
            raw: HttpRequest,
        ) -> Result<HttpResponse> {
            let response = match handcraft_models::inline::#op::Request::from_raw(raw).await {
                Ok(request) => {
                    let response = handlers.#op(request).await;
                    handcraft_models::inline::#op::Responder::to_raw(response)
                }
                Err(e) => handlers.on_bad_request(e),
            };
            actix_web::Result::Ok(response)
        }
    }
}
