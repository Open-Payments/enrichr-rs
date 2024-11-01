use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Enrichable)]
pub fn enrichable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // TODO: Implement the actual macro
    TokenStream::new()
}

pub mod error;
pub mod enrichable;
