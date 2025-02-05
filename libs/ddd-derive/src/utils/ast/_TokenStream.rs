
pub type TokenStream = proc_macro::TokenStream;

macro_rules! tokens {
    ($($tokens:tt)*) => {{
        quote::quote! { $($tokens)* }
            .into()
    }};
}

pub(crate) use tokens;
