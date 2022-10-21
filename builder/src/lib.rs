use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use syn::{DeriveInput, parse_macro_input, Type};
use quote::{quote, format_ident};

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
    let struct_ident = &ast.ident;
    let fields = Field::get_multiple_from_ast(&ast)?;
    // Get identifier for our builder struct. Need for builer impl and original struct's builder method
    let builder_ident = format_ident!("{struct_ident}Builder");
    let builder_struct = expand_builder(&builder_ident, &fields);
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

fn expand_builder(builder_ident: &syn::Ident, fields: &Vec<Field>) -> TokenStream2 {
    let struct_def = expand_builder_struct(&builder_ident, fields);
    let impl_block = expand_builder_impl(builder_ident, fields);
    quote! {
        #struct_def

        #impl_block
    }
}

fn expand_builder_struct(builder_ident: &syn::Ident, fields: &Vec<Field>) -> TokenStream2 {
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
fn expand_builder_impl(builder_ident: &syn::Ident, fields: &Vec<Field>) -> TokenStream2 {
    let fields_init: Vec<TokenStream2> = fields
        .iter()
        // TODO: Do error propogating instead of flat_map (which throws away any errors)
        .flat_map(Field::as_optional_init)
        .collect();
    let field_setters: Vec<TokenStream2> = fields
        .iter()
        .flat_map(Field::as_setter)
        .collect();
    quote! {
        impl #builder_ident {
            fn new() -> #builder_ident {
                #builder_ident {
                    #(#fields_init),*
                }
            }
            #(#field_setters)*
        }
    }
}

struct Field<'a> {
    field_ident: &'a proc_macro2::Ident,
    field_type: &'a Type,
}

impl<'a> Field<'a> {
    fn get_multiple_from_ast(ast: &'a DeriveInput) -> Result<Vec<Self>, syn::Error> {
        println!("{:#?}", ast.data);
        match ast.data {
            syn::Data::Struct(ref data) => {
                match data.fields {
                    syn::Fields::Named(ref name_fields) => {
                        let mut fields = Vec::new();
                        for field in &name_fields.named {
                            fields.push(Self::get_field_from_syn(field)?);
                        }
                        Ok(fields)
                    },
                    syn::Fields::Unnamed(_) => panic!("tuple structs not supported"),
                    syn::Fields::Unit => panic!("unit structs not supported"),
                }
            },
            // TODO: Add proper compiler error message on struct keyword
            syn::Data::Enum(_) => panic!("enum is not supported"),
            syn::Data::Union(_) => panic!("enum is not supported"),
        }
    }
    fn get_field_from_syn(field: &'a syn::Field) -> Result<Self, syn::Error>{
        Ok(Self {
            field_ident: field.ident.as_ref().unwrap(),
            field_type: &field.ty,
        })
    }
    
    fn as_optional_field(&self) -> Result<TokenStream2, syn::Error> {
        let ident = self.field_ident;
        let ty = self.field_type;
        Ok(quote! {
            #ident: std::option::Option<#ty>
        })
    }
    fn as_optional_init(&self) -> Result<TokenStream2, syn::Error> {
        let ident = self.field_ident;
        let _ty = self.field_type;
        Ok(quote! {
            #ident: None
        })
    }
    fn as_setter(&self) -> Result<TokenStream2, syn::Error> {
        let ident = self.field_ident;
        let ty = self.field_type;
        Ok(quote! {
            fn #ident(&mut self, val: #ty) {
                self.#ident = Some(val);
            }
        })
    }
}