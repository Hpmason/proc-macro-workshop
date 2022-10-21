pub(crate) mod expand;
pub(crate) mod structs;


use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let stream = expand::expand(ast)
        .map_err(|err| err.to_compile_error())
        .unwrap();
    stream.into()
}
