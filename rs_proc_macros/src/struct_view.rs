use super::*;
use crate::view_attribute::ImplType;

// Storage struct for everything we need in each impl
pub struct StructViewOptions<'a>
{
    pub name:         Ident,
    pub data_struct:  &'a DataStruct,
    pub attr_options: &'a ViewAttributeOptions
}

// Make View impl
pub struct ImplView<'a>(pub &'a StructViewOptions<'a>);

impl<'a> ToTokens for ImplView<'a>
{
    fn to_tokens(&self, tokens: &mut TS2)
    {
        let struct_name = &self.0.name;
        let table_name = &self.0.attr_options.table_name;
        tokens.append_all(quote! {
                  impl View for #struct_name
                  {
                         const TABLE_NAME: &'static str = #table_name;
                  }
              });
    }
}

// Make FromRow impl
pub struct ImplFromRow<'a>(pub &'a StructViewOptions<'a>);

impl<'a> ToTokens for ImplFromRow<'a>
{
    fn to_tokens(&self, tokens: &mut TS2)
    {
        let struct_name = &self.0.name;

        let r = if let Some(wrapped_struct) = &self.0.attr_options.wrapped
        {
            quote! {
                ( #wrapped_struct ::from_row_opt(row)?)
            }
        }
        else
        {
            let result = fetch::from_row_field_quotes(self.0.data_struct).unwrap();
            quote! { { #result } }
        };
        let content = quote! {
             impl mysql::prelude::FromRow for #struct_name
            {
                fn from_row_opt<'a>(mut row: mysql::Row) -> std::result::Result<Self, mysql::FromRowError> where Self: Sized
                {
                    Ok(Self
                        #r
                    )
                }
            }
        };
        tokens.append_all(content);
    }
}

// Make Updatable Impl
pub struct ImplUpdatable<'a>(pub &'a StructViewOptions<'a>);

impl<'a> ToTokens for ImplUpdatable<'a>
{
    fn to_tokens(&self, tokens: &mut TS2)
    {
        if self.0.attr_options.impl_type != ImplType::Table
        {
            return;
        }

        let struct_name = &self.0.name;
        let r = fetch::to_params_field_quotes(self.0.data_struct).unwrap();
        let content = quote! {
            impl rusty_squirrel::traits::Updatable for #struct_name
            {
                fn to_params(&self) -> mysql::Params
                {
                    use mysql::params;
                    params!(#r)
                }
            }
        };
        tokens.append_all(content);
    }
}

// Make Table Impl
pub struct ImplTable<'a>(pub &'a StructViewOptions<'a>);

impl<'a> ToTokens for ImplTable<'a>
{
    fn to_tokens(&self, tokens: &mut TS2)
    {
        if self.0.attr_options.impl_type != ImplType::Table
        {
            return;
        }
        if let Some(file_name) = &self.0.attr_options.file_name
        {
            let struct_name = &self.0.name;
            tokens.append_all(quote! {
                        impl rusty_squirrel::traits::Table for #struct_name
                        {}
                        impl rusty_squirrel::traits::DbObject for #struct_name
                        {
                            fn create_statement() -> String
                            {
                                use rusty_squirrel::traits::View;
                                include_str!(#file_name).replace("TABLE_NAME", Self::TABLE_NAME)
                            }

                            fn drop_statement() -> String
                            {
                                use rusty_squirrel::traits::View;
                                format!("DROP TABLE IF EXISTS `{}`;", Self::TABLE_NAME)
                            }
                        }
                  });
        }
    }
}

// Make Stored View Impl
pub struct ImplStoredView<'a>(pub &'a StructViewOptions<'a>);

impl<'a> ToTokens for ImplStoredView<'a>
{
    fn to_tokens(&self, tokens: &mut TS2)
    {
        if self.0.attr_options.impl_type != ImplType::StoredView
        {
            return;
        }

        if let Some(file_name) = &self.0.attr_options.file_name
        {
            let struct_name = &self.0.name;
            tokens.append_all(quote! {
                        impl rusty_squirrel::traits::DbObject for #struct_name
                        {
                             fn create_statement() -> String
                            {
                                include_str!(#file_name).to_string()
                            }

                            fn drop_statement() -> String
                            {
                                use rusty_squirrel::traits::View;
                                format!("DROP VIEW IF EXISTS `{}`;", Self::TABLE_NAME)
                            }
                        }
                  });
        }
    }
}
