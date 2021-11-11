use quote::{format_ident, quote, ToTokens};
use syn::{group::parse_parens,
          parse::{Parse, ParseStream},
          punctuated::Punctuated,
          DeriveInput, *};

/// This is a struct to contain the options for splitting field
struct SplitOptions
{
    pub field_name:  LitStr,
    // This is to indicate that when reading field we need an accessor
    pub is_accessor: bool
}

impl Parse for SplitOptions
{
    fn parse(input: ParseStream) -> Result<Self>
    {
        Ok(Self { field_name:  input.parse()?,
                  // Just checking that there are some parens
                  // If it parses then, the is_accessor is true
                  is_accessor: parse_parens(input).is_ok() })
    }
}

struct AttrParams(Punctuated<SplitOptions, Token![,]>);

impl Parse for AttrParams
{
    fn parse(input: ParseStream) -> Result<Self>
    {
        let fields = input.parse_terminated(SplitOptions::parse);
        match fields
        {
            Ok(o) => Ok(Self(o)),
            Err(e) => Err(e)
        }
    }
}

pub fn from_row_field_quotes(ast: &DeriveInput) -> syn::Result<Vec<proc_macro2::TokenStream>>
{
    let mut fqs = vec![];
    if let Data::Struct(ds) = &ast.data
    {
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        for f in &ds.fields
        {
            let field_type = &f.ty;
            let field_ident = &f.ident;
            let string_name = field_ident.clone().expect("3").to_string().replace("r#", "");

            let mut attr_quote: proc_macro2::TokenStream = quote! { row.take_hinted(#string_name)?};

            let mut is_option = false;
            let inner_type = check_and_get_inner("Option", field_type);
            if inner_type.is_some()
            {
                is_option = true; // leaving this so we can do other optional types that also fit in the attribute category
            }

            if is_option
            {
                if inner_type.expect("4")
                             .to_token_stream()
                             .to_string()
                             .contains("DateTime")
                {
                    attr_quote = quote! { row.take_date_time_option(#string_name)?};
                }
            }
            else
            {
                if field_type.to_token_stream().to_string().contains("DateTime")
                {
                    attr_quote = quote! { row.take_date_time(#string_name)?};
                }
            }

            for a in &f.attrs
            {
                match a.path.segments.last().expect("5").ident.to_string().as_str()
                {
                    "rs_e" =>
                    {
                        attr_quote = quote! { row.take_enum(#string_name)?};
                        break;
                    }
                    "rs_spl" =>
                    {
                        if let Ok(params) = a.parse_args_with(AttrParams::parse)
                        {
                            let mut lit_fields = vec![];

                            for lf in params.0
                            {
                                let column_name = format!("{}_{}", string_name, lf.field_name.value());
                                lit_fields.push(column_name)
                            }
                            attr_quote = quote! { #field_type::new(#(row.take_hinted(#lit_fields)?,)*) };
                            break;
                        }
                    }
                    _ => continue
                }
            }

            fqs.push(quote! { #field_ident: #attr_quote,  }.into());
        }
    }
    for fq in fqs.clone().into_iter()
    {
        println!("{}", fq);
    }
    Ok(fqs)
}

pub(crate) fn check_and_get_inner<'a>(outer_type: &str, ty: &'a syn::Type) -> std::option::Option<&'a syn::Type>
{
    if let syn::Type::Path(ref p) = ty
    {
        if p.path.segments.len() < 1 || p.path.segments[0].ident != outer_type
        {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref abga) = p.path.segments[0].arguments
        {
            if abga.args.len() == 0
            {
                println!("no args");
                return None;
            }
            let inner1 = abga.args.first().expect("6");
            if let syn::GenericArgument::Type(ref t) = inner1
            {
                return Some(t);
            }
        }
    }
    None
}

pub fn to_params_field_quotes(ast: &DeriveInput) -> syn::Result<Vec<proc_macro2::TokenStream>>
{
    let mut fqs = vec![];
    if let Data::Struct(ds) = &ast.data
    {
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        for f in &ds.fields
        {
            let field_type = &f.ty;
            let field_ident = &f.ident;
            let string_name = field_ident.clone().expect("3").to_string().replace("r#", "");

            if string_name != "id"
            {
                let mut attr_quote: proc_macro2::TokenStream = quote! { self.#field_ident};

                let mut is_option = false;
                let inner_type = check_and_get_inner("Option", field_type);
                if inner_type.is_some()
                {
                    is_option = true; // leaving this so we can do other optional types that also fit in the attribute category
                }

                if is_option
                {
                    if inner_type.expect("4")
                                 .to_token_stream()
                                 .to_string()
                                 .contains("DateTime")
                    {
                        attr_quote = quote! { self.#field_ident.map(|x| x.format(MYSQL_DATE_FORMAT).to_string()) };
                    }
                }
                else
                {
                    if field_type.to_token_stream().to_string().contains("DateTime")
                    {
                        attr_quote = quote! { self.#field_ident.format(MYSQL_DATE_FORMAT).to_string() };
                    }
                }

                let mut handled_by_attribute = false;

                for a in &f.attrs
                {
                    match a.path.segments.last().expect("5").ident.to_string().as_str()
                    {
                        "rs_e" =>
                        {
                            attr_quote = quote! { (self.#field_ident as u8) };
                            fqs.push(quote! { #string_name => &#attr_quote, }.into());
                            handled_by_attribute = true;
                            break;
                        }
                        "rs_spl" =>
                        {
                            if let Ok(params) = a.parse_args_with(AttrParams::parse)
                            {
                                let mut lit_fields = vec![];
                                for lf in params.0
                                {
                                    let parens = if lf.is_accessor
                                    {
                                        quote! {()}
                                    }
                                    else
                                    {
                                        quote! {}
                                    };
                                    let field_name = format_ident!("{}", lf.field_name.value());
                                    lit_fields.push(quote! { #field_name #parens });
                                }

                                for entry in lit_fields
                                {
                                    let string_quote = format!("{}_{}", string_name, entry);
                                    attr_quote = quote! { self.#field_ident.#entry };
                                    fqs.push(quote! { #string_quote => &#attr_quote, }.into());
                                }
                                handled_by_attribute = true;
                                break;
                            }
                        }
                        _ => continue
                    }
                }
                if !handled_by_attribute
                {
                    fqs.push(quote! { #string_name => &#attr_quote, }.into());
                }
            }
        }
    }
    for fq in fqs.clone().into_iter()
    {
        println!("{}", fq);
    }
    Ok(fqs)
}
