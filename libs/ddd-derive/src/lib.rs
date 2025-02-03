use layout::namespace_mod;
use utils::TokenStream;

namespace_mod!(domain);
namespace_mod!(utils);

#[proc_macro_derive(ValueObject)]
pub fn derive_value_object(tokens: TokenStream) -> TokenStream {
    return domain::derive_value_object(tokens);
}
