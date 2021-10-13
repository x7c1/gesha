use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_delegate_macro(ast: &syn::DeriveInput) -> TokenStream {
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
