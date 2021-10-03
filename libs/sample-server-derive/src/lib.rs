use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro_derive(Sample)]
pub fn delegate_api_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_delegate_macro(&ast)
}

fn impl_delegate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let impl_name = format_ident!("{}GeneratedImpl", name);

    let gen = quote! {
        impl #name {
            pub fn to_api(self) -> #impl_name {
                #impl_name(self)
            }
        }
        struct #impl_name(#name);

        #[async_trait::async_trait]
        impl sample_server::Api for #impl_name {
            async fn index(&self, id: u32, name: String) -> String {
                self.0.index(id, name).await
            }
        }
    };
    gen.into()
}
