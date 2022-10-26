use quote::format_ident;
use syn::{DeriveInput, Type, Ident, spanned::Spanned};

pub(crate) struct Struct<'a> {
    pub original: &'a DeriveInput,
    pub ident: &'a Ident,
    pub fields: Vec<Field<'a>>,
}

pub(crate) struct Field<'a> {
    pub original: &'a syn::Field,
    pub field_ident: &'a Ident,
    pub field_type: &'a Type,
}

impl<'a> Struct<'a> {
    pub fn from_syn(ast: &'a DeriveInput) -> Result<Self, syn::Error> {
        Ok(Self {
            original: ast,
            ident: &ast.ident,
            fields: Field::multiple_from_syn(ast)?,
        })
    }

    pub fn to_builder_ident(&self) -> syn::Ident {
        format_ident!("{}Builder", self.ident)
    }
}

impl<'a> Field<'a> {
    pub(crate) fn multiple_from_syn(ast: &'a DeriveInput) -> Result<Vec<Self>, syn::Error> {
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
                syn::Fields::Unnamed(ref tuple_struct) => Err(syn::Error::new(tuple_struct.span(), "tuple structs not supported")),
                syn::Fields::Unit => Err(syn::Error::new(ast.span(), "unit structs not supported")),
            },
            // TODO: Add proper compiler error message on struct keyword
            syn::Data::Enum(ref data_enum) => Err(syn::Error::new(data_enum.enum_token.span(), "enums are not supported by Builder"))?,
            syn::Data::Union(ref data_union) => Err(syn::Error::new(data_union.union_token.span(), "union structs are not supported by Builder"))?,
        }
    }
    pub(crate) fn get_field_from_syn(field: &'a syn::Field) -> Result<Self, syn::Error> {
        Ok(Self {
            original: field,
            field_ident: field.ident.as_ref().unwrap(),
            field_type: &field.ty,
        })
    }
}
