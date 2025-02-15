#![allow(clippy::all)]
#![allow(non_snake_case)]

use modules::*;
use utils::TokenStream;

namespace!(domain);
namespace!(utils);

#[proc_macro_derive(Entity, attributes(ddd))]
pub fn deriveEntity(tokens: TokenStream) -> TokenStream {
    return domain::deriveEntity(tokens);
}

#[proc_macro_derive(Identifier)]
pub fn deriveIdentifier(tokens: TokenStream) -> TokenStream {
    return domain::deriveIdentifier(tokens);
}

#[proc_macro_derive(ValueObject)]
pub fn deriveValueObject(tokens: TokenStream) -> TokenStream {
    return domain::deriveValueObject(tokens);
}
