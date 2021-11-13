#![allow(clippy::suspicious_else_formatting)]
use crate::{struct_view::{ImplFromRow, ImplTable, ImplTableCreate, ImplUpdatable, ImplView, StructViewOptions},
            view_attribute::ViewAttributeOptions};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::*;
use syn::{parse::{Parse, ParseStream},
          spanned::Spanned,
          *};

mod fetch;
mod struct_view;
mod view_attribute;
// Custom Keywords
mod custom_key_words
{
    syn::custom_keyword!(table);
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
                                 .expect("1")
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
        let impl_table_create = ImplTableCreate(&opts);
        let impl_updatable = ImplUpdatable(&opts);
        use convert_case::{Case, Casing};
        let mod_name = format_ident!("impl_{}", opts.name.to_string().to_case(Case::Snake));

        let extended = quote! {
            mod #mod_name
            {
                use super::*;
                use rusty_squirrel::traits::{Taker, View};

                #impl_view

                #impl_from_row

                #impl_table

                #impl_table_create

                #impl_updatable
            };
        };
        println!("{}", extended);

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
        use convert_case::{Case, Casing};
        let mod_name = format_ident!("impl_{}", ast.ident.to_string().to_case(Case::Snake));

        let mut extended = quote! {};
        for v in &e.variants
        {
            let variant_type = &v;

            extended.append_all(quote! {
                        mod #mod_name
                        {
                            use super::*;
                            use anyhow::Error;

                            impl TryFrom<u8> for #variant_type //(enums field)
                            {
                                type Error = anyhow::Error;

                                fn try_from(value: u8) -> Result<Self, Self::Error>
                                {
                                    Ok((match value
                                        {
                                            (x if x == <self>::#variant_type as u8 => <self>::#variant_type,)*
                                            _ => anyhow::bail!("Invalid Enum Value")
                                        }))
                                }

                            }
                        }
                    });
        }
        println!("{}", extended);
        extended.into()
    }
    else
    {
        derive_error!(ast, "Expected Enum");
    }
}
