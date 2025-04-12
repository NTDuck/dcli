mod behaviours;
mod interfaces;
mod utils;

use proc_macro::TokenStream;

#[proc_macro_derive(New)]
pub fn derive_new(tokens: TokenStream) -> TokenStream {
    crate::behaviours::derive_new(tokens)
}

#[proc_macro_derive(DataTransferObjectWithoutDeserialize)]
pub fn derive_data_transfer_object_without_deserialize(tokens: TokenStream) -> TokenStream {
    crate::interfaces::derive_data_transfer_object_without_deserialize(tokens)
}

#[proc_macro_derive(DataTransferObjectWithoutSerde)]
pub fn derive_data_transfer_object_without_serde(tokens: TokenStream) -> TokenStream {
    crate::interfaces::derive_data_transfer_object_without_serde(tokens)
}

#[proc_macro_derive(Entity, attributes(axiom))]
pub fn derive_entity(tokens: TokenStream) -> TokenStream {
    crate::interfaces::ddd::domain::derive_entity(tokens)
}

#[proc_macro_derive(Identifier)]
pub fn derive_identifier(tokens: TokenStream) -> TokenStream {
    crate::interfaces::ddd::domain::derive_identifier(tokens)
}

#[proc_macro_derive(ValueObject)]
pub fn derive_value_object(tokens: TokenStream) -> TokenStream {
    crate::interfaces::ddd::domain::derive_value_object(tokens)
}
