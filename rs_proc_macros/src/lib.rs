#![allow(clippy::suspicious_else_formatting)]
use crate::{enum_view::{EnumViewOptions, ImplU8},
            struct_view::{ImplFromRow, ImplStoredView, ImplTable, ImplUpdatable, ImplView, StructViewOptions},
            view_attribute::ViewAttributeOptions};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::*;
use syn::{parse::*, punctuated::Punctuated, spanned::Spanned, *};

mod enum_view;
mod fetch;
mod struct_view;
mod view_attribute;
// Custom Keywords
mod kw
{
    syn::custom_keyword!(table);
    syn::custom_keyword!(stored_view);
    syn::custom_keyword!(attr);
    syn::custom_keyword!(wrapped);
}

// Helper macro to spit out error in main derive fn
macro_rules! derive_error {
    ($ast:ident, $m:literal) => {
        return Error::new($ast.span(), $m).to_compile_error().into();
    };
    ($e:ident) => {
        return $e.to_compile_error().into()
    };
}

#[proc_macro_derive(RustyParams, attributes(rs_e, rs_spl, rs_view))]
pub fn derive(input: TokenStream) -> TokenStream
{
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(ds) = &ast.data
    {
        if !ast.attrs.iter().any(|x| x.path.is_ident("rs_view"))
        {
            derive_error!(ast, "Expected rs_view attribute");
        }

        let main_attr = match ast.attrs
                                 .iter()
                                 .filter(|x| x.path.is_ident("rs_view"))
                                 .last()
                                 .unwrap()
                                 .parse_args_with(ViewAttributeOptions::parse)
        {
            Ok(x) => x,
            Err(e) => derive_error!(e)
        };

        let opts = StructViewOptions { name:         ast.ident,
                                       data_struct:  ds,
                                       attr_options: &main_attr };

        let impl_view = ImplView(&opts);
        let impl_from_row = ImplFromRow(&opts);
        let impl_table = ImplTable(&opts);
        let impl_updatable = ImplUpdatable(&opts);
        let impl_stored_view = ImplStoredView(&opts);

        use convert_case::{Case, Casing};
        let mod_name: Ident = format_ident!("impl_{}", opts.name.to_string().to_case(Case::Snake));

        let custom_attr = main_attr.attr
                                   .as_ref()
                                   .map(|x| quote! {#[#x]})
                                   .unwrap_or_else(|| quote! {});

        let extended = quote! {

            #custom_attr
            mod #mod_name
            {
                use super::*;
                use rusty_squirrel::traits::{Taker, View};

                #impl_view

                #impl_from_row

                #impl_table

                #impl_updatable

                #impl_stored_view
            };
        };
        //println!("{}", extended);
        extended.into()
    }
    else
    {
        derive_error!(ast, "Expected Struct");
    }
}

#[proc_macro_derive(RustyEnum)]
pub fn from_enum(input: TokenStream) -> TokenStream
{
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    if let Data::Enum(e) = &ast.data
    {
        let enum_name = ast.ident;

        let opts = EnumViewOptions { name:     enum_name.clone(),
                                     variants: e.variants.clone() };
        let impl_from_u8 = ImplU8(&opts);

        use convert_case::{Case, Casing};
        let mod_ident: Ident = format_ident!("impl_{}", enum_name.to_string().to_case(Case::Snake));

        let mut extended = quote! {};

        extended.append_all(quote! {
                    mod #mod_ident
                    {
                        use super::*;
                        use anyhow::Error;

                        #impl_from_u8
                    }
                });
        //println!("{}", extended);
        extended.into()
    }
    else
    {
        derive_error!(ast, "Expected Enum");
    }
}
