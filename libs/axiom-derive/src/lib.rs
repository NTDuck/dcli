#![allow(nonstandard_style)]

use layout::*;

namespace!(behaviours);
namespace!(interfaces);
namespace!(utils);

use proc_macro::TokenStream;

#[proc_macro_derive(New)]
pub fn derive_New(tokens: TokenStream) -> TokenStream {
    return crate::behaviours::derive_New(tokens);
}

#[proc_macro_derive(NewType)]
pub fn derive_NewType(tokens: TokenStream) -> TokenStream {
    return crate::behaviours::derive_NewType(tokens);
}

#[proc_macro_derive(DataTransferObjectWithoutDeserialize)]
pub fn deriveDataTransferObjectWithoutDeserialize(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::derive_DataTransferObjectWithoutDeserialize(tokens);
}

#[proc_macro_derive(DataTransferObjectWithoutSerde)]
pub fn deriveDataTransferObjectWithoutSerde(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::derive_DataTransferObjectWithoutSerde(tokens);
}

#[proc_macro_derive(Entity, attributes(axiom))]
pub fn deriveEntity(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::ddd::domain::derive_Entity(tokens);
}

#[proc_macro_derive(Identifier)]
pub fn deriveIdentifier(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::ddd::domain::derive_Identifier(tokens);
}

#[proc_macro_derive(ValueObject)]
pub fn deriveValueObject(tokens: TokenStream) -> TokenStream {
    return crate::interfaces::ddd::domain::derive_ValueObject(tokens);
}
