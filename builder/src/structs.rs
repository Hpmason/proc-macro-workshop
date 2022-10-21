use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, Type};

pub(crate) struct Field<'a> {
    field_ident: &'a proc_macro2::Ident,
    field_type: &'a Type,
}

impl<'a> Field<'a> {
    pub(crate) fn get_multiple_from_ast(ast: &'a DeriveInput) -> Result<Vec<Self>, syn::Error> {
        // println!("{:#?}", ast.data);
        match ast.data {
            syn::Data::Struct(ref data) => match data.fields {
                syn::Fields::Named(ref name_fields) => {
                    let mut fields = Vec::new();
                    for field in &name_fields.named {
                        fields.push(Self::get_field_from_syn(field)?);
                    }
                    Ok(fields)
                }
                syn::Fields::Unnamed(_) => panic!("tuple structs not supported"),
                syn::Fields::Unit => panic!("unit structs not supported"),
            },
            // TODO: Add proper compiler error message on struct keyword
            syn::Data::Enum(_) => panic!("enum is not supported"),
            syn::Data::Union(_) => panic!("enum is not supported"),
        }
    }
    pub(crate) fn get_field_from_syn(field: &'a syn::Field) -> Result<Self, syn::Error> {
        Ok(Self {
            field_ident: field.ident.as_ref().unwrap(),
            field_type: &field.ty,
        })
    }

    pub(crate) fn as_optional_field(&self) -> Result<TokenStream2, syn::Error> {
        let ident = self.field_ident;
        let ty = self.field_type;
        Ok(quote! {
            #ident: std::option::Option<#ty>
        })
    }
    pub(crate) fn as_optional_init(&self) -> Result<TokenStream2, syn::Error> {
        let ident = self.field_ident;
        Ok(quote! {
            #ident: None
        })
    }
    pub(crate) fn as_build_init(&self) -> Result<TokenStream2, syn::Error> {
        let ident = self.field_ident;
        let clone_statement = quote_spanned!(ident.span()=>
            #ident.clone()
        );
        Ok(quote! {
            #ident: self.#clone_statement.ok_or(String::from(concat!("{} was not set", stringify!(#ident))))?
        })
    }
    pub(crate) fn as_setter(&self) -> Result<TokenStream2, syn::Error> {
        let ident = self.field_ident;
        let ty = self.field_type;
        Ok(quote! {
            fn #ident(&mut self, val: #ty) -> &mut Self {
                self.#ident = Some(val);
                self
            }
        })
    }
}
