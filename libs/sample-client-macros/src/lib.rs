use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(DelegateSample)]
pub fn delegate_sample_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_delegate_macro(&ast)
}

fn impl_delegate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[async_trait::async_trait]
        impl sample_client::Api for #name {
            async fn index(&self, id: u32, name: String) -> String {
                #name::index(self, id, name).await
            }
        }
    };
    gen.into()
}
