pub(crate) mod expand;
pub(crate) mod structs;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let stream = expand::expand(ast)
        .unwrap_or_else(syn::Error::into_compile_error);
    stream.into()
}
