pub(crate) type TokenStream = proc_macro::TokenStream;

macro_rules! tokenize {
    ($($tokens:tt)*) => {{
        quote::quote! { $($tokens)* }
            .into()
    }};
}

pub(crate) use tokenize;
