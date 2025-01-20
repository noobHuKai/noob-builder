use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
mod  builder;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder_with_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    builder::BuilderContext::from(input)
        .render()
        .into()
}