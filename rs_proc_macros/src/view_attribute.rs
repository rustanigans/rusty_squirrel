use super::*;
use syn::group::{parse_parens, Parens};

pub struct ViewAttributeOptions
{
    pub table_name: LitStr,
    pub impl_table: bool,
    pub file_name:  Option<LitStr>,
    pub attr:       Option<TS2>
}

impl Parse for ViewAttributeOptions
{
    fn parse(input: ParseStream) -> Result<Self>
    {
        let table_name = input.parse()?;
        input.parse::<Token![,]>().ok();
        let impl_table = input.parse::<custom_key_words::table>().is_ok();

        let file_name = if impl_table
        {
            let Parens { content, .. } = parse_parens(input)?;
            Some(content.parse::<LitStr>()?)
        }
        else
        {
            None
        };

        let attr = if input.parse::<custom_key_words::attr>().is_ok()
        {
            let Parens { content, .. } = parse_parens(input)?;
            Some(content.parse()?)
        }
        else
        {
            None
        };

        Ok(Self { table_name,
                  impl_table,
                  file_name,
                  attr })
    }
}
