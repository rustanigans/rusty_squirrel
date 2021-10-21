#[macro_export]
macro_rules! impl_try_from_for_enum {
    ($t: ty, $u: ty, $($arg:ident),*) => {
        impl TryFrom<$u> for $t
        {
            type Error = anyhow::Error;

            fn try_from(value: $u) -> Result<Self, Self::Error>
            {
                Ok((match value
                {
                   $(
                        x if x == <$t>::$arg as $u => <$t>::$arg,
                   )*
                    _ => anyhow::bail!("Invalid Enum Value")
                }))
            }
        }
    }
}
