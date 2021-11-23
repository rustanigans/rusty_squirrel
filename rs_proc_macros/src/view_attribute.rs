use super::*;
use syn::group::{parse_parens, Parens};

#[derive(PartialOrd, PartialEq)]
pub enum ImplType
{
    None,
    Table,
    StoredView
}

pub struct ViewAttributeOptions
{
    pub table_name: LitStr,
    pub impl_type:  ImplType,
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
        let impl_stored_view = input.parse::<custom_key_words::stored_view>().is_ok();

        let file_name = if impl_table | impl_stored_view
        {
            let Parens { content, .. } = parse_parens(input)?;
            Some(content.parse::<LitStr>()?)
        }
        else
        {
            None
        };
        let impl_type = if impl_table
        {
            ImplType::Table
        }
        else if impl_stored_view
        {
            ImplType::StoredView
        }
        else
        {
            ImplType::None
        };

        input.parse::<Token![,]>().ok();
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
                  impl_type,
                  file_name,
                  attr })
    }
}
