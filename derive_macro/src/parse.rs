use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Error, DataStruct, Field, Ident, Generics};

pub(crate) fn expand(derive_input: DeriveInput) -> syn::Result<TokenStream> {
    match derive_input.data {
        Data::Struct(data_struct) => Ok(expand_data_struct(derive_input.ident, derive_input.generics, data_struct)),
        Data::Enum(..) => Err(Error::new_spanned(derive_input, "Enums are not supported")),
        Data::Union(..) => Err(Error::new_spanned(derive_input, "Unions are not supported")),
    }
}

fn expand_data_struct(name: Ident, Generics { params, .. }: Generics, data_struct: DataStruct) -> TokenStream {
    let fields = data_struct.fields.iter().map(|Field { ident, ..}| {
        quote!(#ident: cursor.parse()?)
    });
    
    let lt = if !params.is_empty() {
        Some(quote!('source))
    } else { None };

    quote! {
        impl<'source> Parse<'source> for #name<#lt> {
            fn parse<I: Index<usize, Output = Token<'source>>>(cursor: &mut Cursor<'source, I>) -> Result<'source, Self> {
                Ok(
                    Self {
                        #(#fields),*
                    }
                )
            }
        }
    }
}