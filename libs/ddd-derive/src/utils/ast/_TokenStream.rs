pub type TokenStream = proc_macro::TokenStream;
pub type IntermediateTokenStream = proc_macro2::TokenStream;

macro_rules! tokenize {
    ($($tokens:tt)*) => {{
        quote::quote! { $($tokens)* }
            .into()
    }};
}

pub(crate) use tokenize;

pub use quote::quote as implementation;
