use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Error, DataStruct, Field, Ident, Generics, Expr};

pub(crate) fn expand(derive_input: DeriveInput) -> syn::Result<TokenStream> {
    match derive_input.data {
        Data::Struct(data_struct) => Ok(expand_data_struct(derive_input.ident, derive_input.generics, data_struct)?),
        Data::Enum(..) => Err(Error::new_spanned(derive_input, "Enums are not supported")),
        Data::Union(..) => Err(Error::new_spanned(derive_input, "Unions are not supported")),
    }
}

fn expand_data_struct(name: Ident, Generics { params, .. }: Generics, data_struct: DataStruct) -> syn::Result<TokenStream> {
    let fields = data_struct.fields.iter().map(|Field { ident, attrs, ty, ..}| -> syn::Result<TokenStream> {
        if let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("parse_if")) {
            let test = attr.parse_args::<Expr>()?;

            Ok(quote! {
                let #ident: #ty = if #test {
                    Some(cursor.parse()?)
                } else { None }
            })
        } else {
            Ok(quote!(let #ident: #ty = cursor.parse()?))
        }
    });
    let fields = fields.collect::<Result<Vec<_>, _>>()?;
    
    let raw_field_idents = data_struct.fields.iter().map(|field| &field.ident);

    let lt = if !params.is_empty() {
        Some(quote!('source))
    } else { None };

    Ok(quote! {
        impl<'source> crate::parser::Parse<'source> for #name<#lt> {
            fn parse<I: std::ops::Index<usize, Output = crate::lexer::token::Token<'source>>>(cursor: &mut crate::parser::cursor::Cursor<'source, I>) -> crate::common::error::Result<'source, Self> {
                #(#fields);*;
                Ok(
                    Self {
                        #(#raw_field_idents),*
                    }
                )
            }
        }
    })
}