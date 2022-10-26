use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, spanned::Spanned};

use crate::structs::Struct;

pub(crate) fn expand(ast: DeriveInput) -> Result<TokenStream2, syn::Error> {
    let struct_data = Struct::from_syn(&ast)?;
    // Get identifier for our builder struct. Need for builer impl and original struct's builder method

    let base_impl = expand_base_struct_impl(&struct_data);
    let builder_struct = expand_builder_struct(&struct_data);
    let builder_impl = expand_builder(&struct_data);
    let expanded = quote! {
        #base_impl

        #builder_struct

        #builder_impl
    };
    Ok(expanded)
}

fn expand_base_struct_impl(struct_data: &Struct) -> TokenStream2 {
    let ident = struct_data.ident;
    let builder_ident = struct_data.to_builder_ident();
    quote! {
        impl #ident {
            fn builder() -> #builder_ident {
                #builder_ident::new()
            }
        }
    }
}

fn expand_builder_struct(struct_data: &Struct) -> TokenStream2 {
    let builder_ident = struct_data.to_builder_ident();
    let inner_fields: Vec<TokenStream2>  = struct_data.fields
        .iter()
        .map(|field| {
            let ident = field.field_ident;
            let ty = field.field_type;
            quote_spanned! {field.original.span()=>
                #ident: std::option::Option<#ty>
            }
        })
        .collect();
    quote! {
        struct #builder_ident {
            #(#inner_fields),*
        }
    }
}

fn expand_builder(
    struct_data: &Struct,
) -> TokenStream2 {
    let impl_block = expand_builder_impl(struct_data);
    
    quote! {
        #impl_block
    }
}

fn expand_builder_impl(
    struct_data: &Struct,
) -> TokenStream2 {
    let fields_init: Vec<TokenStream2> = struct_data.fields
        .iter()
        // TODO: Do error propogating instead of flat_map (which throws away any errors)
        .map(|field| {
            let ident = field.field_ident;
            quote_spanned! {field.original.span()=>
                #ident: None
            }
        }).collect();

    let field_setters: Vec<TokenStream2> = struct_data.fields
        .iter()
        // TODO: Do error propogating instead of flat_map (which throws away any errors)
        .map(|field| {
            let ident = field.field_ident;
            let ty = field.field_type;

            let assert_clone = quote_spanned! {ty.span()=>
                struct _AssertClone where #ty: Clone;
            };
        
            quote_spanned! {field.original.span()=>
                fn #ident(&mut self, val: #ty) -> &mut Self {
                    #assert_clone
                    self.#ident = Some(val);
                    self
                }
            }
        }).collect();
    
    let build_fn = expand_build_method(struct_data);

    let builder_ident = struct_data.to_builder_ident();
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
    struct_data: &Struct
) -> TokenStream2 {
    let field_inits: Vec<TokenStream2> = struct_data.fields.iter().map(|field| {
        let ident = field.field_ident;

        quote_spanned! {field.original.span()=>
            #ident: self.#ident.clone().ok_or(String::from(concat!("{} was not set", stringify!(#ident))))?
        }
    }).collect();
    let original_ident = struct_data.ident;
    quote! {
        fn build(&mut self) -> Result<#original_ident, Box<dyn std::error::Error>> {
            Ok(#original_ident {
                #(#field_inits),*
            })
        }
    }
}