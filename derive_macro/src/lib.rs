use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

mod parse;

#[proc_macro_derive(Parse)]
pub fn parse(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    parse::expand(derive_input).unwrap_or_else(Error::into_compile_error).into()
}
