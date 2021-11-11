#![allow(clippy::suspicious_else_formatting)]
#![allow(unused_imports)]

mod fetch;

use proc_macro::{TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::{Parse, ParseStream, Parser},
          parse_macro_input,
          punctuated::Punctuated,
          token::Colon2,
          Attribute, DeriveInput, Expr, LitStr, PathSegment, Result, Token};

#[proc_macro_derive(RustyParams, attributes(rs_e, rs_spl))]
pub fn derive(input: TokenStream) -> TokenStream
{
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let field_quote: Vec<proc_macro2::TokenStream> = fetch::field_quotes(&ast).expect("1");

    let extended = quote! {
        impl mysql::prelude::FromRow for #name
        {
            fn from_row_opt<'a>(mut row: mysql::Row) -> std::result::Result<Self, mysql::FromRowError> where Self: Sized
            {
                Ok(Self
                {
                    #(#field_quote)*
                })
            }
        }
    };
    extended.into()
}
