#![allow(non_snake_case)]

use layout::*;

namespace!(behaviours);
namespace!(interfaces);
namespace!(utils);

use proc_macro::TokenStream;

#[proc_macro_derive(New)]
pub fn deriveNew(tokens: TokenStream) -> TokenStream {
    return crate::behaviours::deriveNew(tokens);
}

#[proc_macro_derive(NewType)]
pub fn deriveNewType(tokens: TokenStream) -> TokenStream {
    return crate::behaviours::deriveNewType(tokens);
}

#[proc_macro_derive(SerdelessDataTransferObject)]
pub fn deriveSerdelessDataTransferObject(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::deriveSerdelessDataTransferObject(tokens);
}

#[proc_macro_derive(Entity, attributes(axiom))]
pub fn deriveEntity(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::ddd::domain::deriveEntity(tokens);
}

#[proc_macro_derive(Identifier)]
pub fn deriveIdentifier(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::ddd::domain::deriveIdentifier(tokens);
}

#[proc_macro_derive(ValueObject)]
pub fn deriveValueObject(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::ddd::domain::deriveValueObject(tokens);
}
