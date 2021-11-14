use super::*;
use syn::token::Comma;

pub struct EnumViewOptions
{
    pub name:     Ident,
    pub variants: Punctuated<Variant, Comma>
}

pub struct ImplU8<'a>(pub &'a EnumViewOptions);

impl<'a> ToTokens for ImplU8<'a>
{
    fn to_tokens(&self, tokens: &mut TS2)
    {
        let enum_name = &self.0.name;
        let mut variant_idents = TS2::new();

        for i in 0..self.0.variants.len()
        {
            let variant_ident = &self.0.variants[i].clone().ident;
            let number = i as u8;
            variant_idents.append_all(quote! { #number => #enum_name::#variant_ident, });
        }

        let content = quote! {
            impl TryFrom<u8> for #enum_name
            {
                type Error = anyhow::Error;

                fn try_from(value: u8) -> std::result::Result<Self, Self::Error>
                {
                    Ok(match value
                    {
                        #variant_idents
                    _ => anyhow::bail!("Invalid Enum Value")
                    })
                }
            }
        };
        tokens.append_all(content);
    }
}
