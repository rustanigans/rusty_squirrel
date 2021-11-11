#![allow(clippy::suspicious_else_formatting)]

mod fetch;

use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::{Parse, Parser},
          parse_macro_input, DeriveInput};

#[proc_macro_derive(RustyParams, attributes(rs_e, rs_spl))]
pub fn derive(input: TokenStream) -> TokenStream
{
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let from_row_field_quote: Vec<proc_macro2::TokenStream> = fetch::from_row_field_quotes(&ast).expect("1a");
    let to_params_field_quote: Vec<proc_macro2::TokenStream> = fetch::to_params_field_quotes(&ast).expect("1b");

    let extended = quote! {

        impl Updatable for #name
        {
            fn to_params(&self) -> Params
            {
                params!(
                    #(#to_params_field_quote)*
                )
            }
        }

        impl mysql::prelude::FromRow for #name
        {
            fn from_row_opt<'a>(mut row: mysql::Row) -> std::result::Result<Self, mysql::FromRowError> where Self: Sized
            {
                Ok(Self
                {
                    #(#from_row_field_quote)*
                })
            }
        }
    };
    extended.into()
}