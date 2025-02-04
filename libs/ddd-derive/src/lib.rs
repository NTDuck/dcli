use modules::*;
use utils::TokenStream;

namespace!(domain);
namespace!(utils);

#[proc_macro_derive(Entity, attributes(ddd))]
pub fn derive_entity(tokens: TokenStream) -> TokenStream { return domain::derive_entity(tokens); }

#[proc_macro_derive(Identifier)]
pub fn derive_identifier(tokens: TokenStream) -> TokenStream {
    return domain::derive_identifier(tokens);
}

#[proc_macro_derive(ValueObject)]
pub fn derive_value_object(tokens: TokenStream) -> TokenStream {
    return domain::derive_value_object(tokens);
}
