mod delegator;
use delegator::impl_delegate_macro;

mod extractor;
use extractor::validate_signature;

mod reified;

use proc_macro::TokenStream;

#[proc_macro_derive(Sample)]
pub fn delegate_api_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_delegate_macro(&ast).into()
}

#[proc_macro_attribute]
pub fn define(attr: TokenStream, item: TokenStream) -> TokenStream {
    let operation = attr.to_string();
    validate_signature(&operation, item.clone().into());
    item
}
