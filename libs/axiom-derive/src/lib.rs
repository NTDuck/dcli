#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

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

#[proc_macro_derive(DataTransferObjectWithoutSerde)]
pub fn deriveDataTransferObjectWithoutSerde(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::deriveDataTransferObjectWithoutSerde(tokens);
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
