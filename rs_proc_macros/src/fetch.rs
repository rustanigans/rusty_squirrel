use super::*;
use syn::punctuated::Punctuated;

struct AttrParams(Punctuated<LitStr, Token![,]>);

impl Parse for AttrParams
{
    fn parse(input: ParseStream) -> Result<Self>
    {
        let fields = input.parse_terminated(Parse::parse);
        match fields
        {
            Ok(o) => Ok(Self(o)),
            Err(e) => Err(e)
        }
    }
}

pub fn from_row_field_quotes(ds: &DataStruct) -> syn::Result<TS2>
{
    let mut fqs = TS2::new();

    for f in &ds.fields
    {
        let field_type = &f.ty;
        let field_ident = &f.ident;
        let string_name = field_ident.clone().expect("2").to_string().replace("r#", "");

        let mut attr_quote: TS2 = quote! { row.take_hinted(#string_name)?};

        let mut is_option = false;
        let inner_type = check_and_get_inner("Option", field_type);
        if inner_type.is_some()
        {
            is_option = true; // leaving this so we can do other optional types that also fit in the attribute category
        }

        if is_option
        {
            if inner_type.expect("3")
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
            match a.path.segments.last().expect("4").ident.to_string().as_str()
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
                            let column_name = format!("{}_{}", string_name, lf.value());
                            lit_fields.push(column_name)
                        }
                        attr_quote = quote! { #field_type::new(#(row.take_hinted(#lit_fields)?,)*) };
                        break;
                    }
                }
                _ => continue
            }
        }
        fqs.append_all(quote! { #field_ident: #attr_quote,  });
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
            let inner1 = abga.args.first().expect("5");
            if let syn::GenericArgument::Type(ref t) = inner1
            {
                return Some(t);
            }
        }
    }
    None
}

pub fn to_params_field_quotes(ds: &DataStruct) -> syn::Result<TS2>
{
    let mut fqs = TS2::new();

    for f in &ds.fields
    {
        let field_type = &f.ty;
        let field_ident = &f.ident;
        let string_name = field_ident.clone().expect("6").to_string().replace("r#", "");

        if string_name != "id"
        {
            let mut attr_quote: TS2 = quote! { self.#field_ident};

            let mut is_option = false;
            let inner_type = check_and_get_inner("Option", field_type);
            if inner_type.is_some()
            {
                is_option = true; // leaving this so we can do other optional types that also fit in the attribute category
            }

            if is_option
            {
                if inner_type.expect("7")
                             .to_token_stream()
                             .to_string()
                             .contains("DateTime")
                {
                    attr_quote =
                        quote! { self.#field_ident.map(|x| x.format(rusty_squirrel::MYSQL_DATE_FORMAT).to_string()) };
                }
            }
            else
            {
                if field_type.to_token_stream().to_string().contains("DateTime")
                {
                    attr_quote = quote! { self.#field_ident.format(rusty_squirrel::MYSQL_DATE_FORMAT).to_string() };
                }
            }

            let mut handled_by_attribute = false;

            for a in &f.attrs
            {
                match a.path.segments.last().expect("8").ident.to_string().as_str()
                {
                    "rs_e" =>
                    {
                        attr_quote = quote! { (self.#field_ident as u8) };
                        fqs.append_all(quote! { #string_name => &#attr_quote, });
                        handled_by_attribute = true;
                        break;
                    }
                    "rs_spl" =>
                    {
                        if let Ok(params) = a.parse_args_with(AttrParams::parse)
                        {
                            for lf in params.0
                            {
                                let string_quote = format!("{}_{}", string_name, lf.value());
                                let field_name = format_ident!("{}", lf.value());
                                attr_quote = quote! { self.#field_ident.#field_name };

                                fqs.append_all(quote! { #string_quote => &#attr_quote (), });
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
                fqs.append_all(quote! { #string_name => &#attr_quote, });
            }
        }
    }
    Ok(fqs)
}
