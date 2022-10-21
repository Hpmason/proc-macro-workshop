use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::structs::Field;

pub(crate) fn expand(ast: DeriveInput) -> Result<TokenStream2, syn::Error> {
    // println!("{:#?}", ast);
    let struct_ident = &ast.ident;
    let fields = Field::get_multiple_from_ast(&ast)?;
    // Get identifier for our builder struct. Need for builer impl and original struct's builder method
    let builder_ident = format_ident!("{struct_ident}Builder");
    let builder_struct = expand_builder(struct_ident, &builder_ident, &fields);
    let expanded = quote! {
        impl #struct_ident {
            fn builder() -> #builder_ident {
                #builder_ident::new()
            }
        }
        #builder_struct
    };
    Ok(expanded)
}

fn expand_builder(
    original_ident: &syn::Ident,
    builder_ident: &syn::Ident,
    fields: &[Field],
) -> TokenStream2 {
    let struct_def = expand_builder_struct(builder_ident, fields);
    let impl_block = expand_builder_impl(original_ident, builder_ident, fields);

    quote! {
        #struct_def

        #impl_block
    }
}

fn expand_builder_impl(
    original_ident: &syn::Ident,
    builder_ident: &syn::Ident,
    fields: &[Field],
) -> TokenStream2 {
    let fields_init: Vec<TokenStream2> = fields
        .iter()
        // TODO: Do error propogating instead of flat_map (which throws away any errors)
        .flat_map(Field::as_optional_init)
        .collect();
    let field_setters: Vec<TokenStream2> = fields.iter().flat_map(Field::as_setter).collect();
    let build_fn = expand_build_method(original_ident, builder_ident, fields);
    quote! {
        impl #builder_ident {
            fn new() -> #builder_ident {
                #builder_ident {
                    #(#fields_init),*
                }
            }

            #build_fn

            #(#field_setters)*
        }
    }
}

fn expand_build_method(
    original_ident: &syn::Ident,
    _builder_ident: &syn::Ident,
    fields: &[Field],
) -> TokenStream2 {
    let field_inits: Vec<TokenStream2> = fields.iter().flat_map(Field::as_build_init).collect();
    quote! {
        fn build(&mut self) -> Result<#original_ident, Box<dyn std::error::Error>> {
            Ok(#original_ident {
                #(#field_inits),*
            })
        }
    }
}

fn expand_builder_struct(builder_ident: &syn::Ident, fields: &[Field]) -> TokenStream2 {
    let builder_fields: Vec<TokenStream2> = fields
        .iter()
        // TODO: Do error propogating instead of flat_map (which throws away any errors)
        .flat_map(Field::as_optional_field)
        .collect();
    quote! {
        struct #builder_ident {
            #(#builder_fields),*
        }
    }
}
