use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let stream = expand(ast)
        .map_err(|err| err.to_compile_error())
        .unwrap();
    stream.into()
}


fn expand(ast: DeriveInput) -> Result<TokenStream2, syn::Error> {
    // println!("{:#?}", ast);
    let struct_ident = ast.ident;
    let expanded = quote! {
        impl #struct_ident {
            fn builder() -> () {
                ()
            }
        }
    };
    Ok(expanded)
}