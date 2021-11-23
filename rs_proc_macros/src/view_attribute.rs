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
    pub attr:       Option<TS2>,
    pub wrapped:    Option<Ident>
}

impl Parse for ViewAttributeOptions
{
    fn parse(input: ParseStream) -> Result<Self>
    {
        let table_name = input.parse()?;
        input.parse::<Token![,]>().ok();
        let impl_table = input.parse::<kw::table>().is_ok();
        let impl_stored_view = input.parse::<kw::stored_view>().is_ok();

        let (file_name, wrapped) = if impl_table | impl_stored_view
        {
            let Parens { content, .. } = parse_parens(input)?;
            let file_name = content.parse::<LitStr>()?;
            content.parse::<Token![,]>().ok();
            let wrapped = if content.parse::<kw::wrapped>().is_ok()
            {
                let Parens { content, .. } = parse_parens(&content)?;
                Some(content.parse::<Ident>()?)
            }
            else
            {
                None
            };
            (Some(file_name), wrapped)
        }
        else
        {
            (None, None)
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
        let attr = if input.parse::<kw::attr>().is_ok()
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
                  attr,
                  wrapped })
    }
}
