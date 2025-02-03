use layout::namespace_mod;
use proc_macro::TokenStream;

namespace_mod!(domain);

#[proc_macro_derive(ValueObject)]
pub fn derive_value_object(input: TokenStream) -> TokenStream {
    domain::derive_value_object(input)
}
