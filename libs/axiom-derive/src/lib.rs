#![allow(non_snake_case)]
#![allow(unused_variables)]

use layout::*;

namespace!(behaviours);
namespace!(interfaces);

#[proc_macro_derive(New)]
pub fn deriveNew(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    return crate::behaviours::deriveNew(tokens);
}

// #[proc_macro_derive(NewType)]
// pub fn deriveNewType(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     return crate::behaviours::deriveNewType(tokens);
// }

// #[proc_macro_derive(DataTransferObject)]
// pub fn deriveDataTransferObject(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     return crate::interfaces::deriveDataTransferObject(tokens);
// }

// #[proc_macro_derive(Entity)]
// pub fn deriveEntity(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     return crate::interfaces::ddd::domain::deriveEntity(tokens);
// }

// #[proc_macro_derive(Identifier)]
// pub fn deriveIdentifier(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     return crate::interfaces::ddd::domain::deriveIdentifier(tokens);
// }

// #[proc_macro_derive(ValueObject)]
// pub fn deriveValueObject(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     return crate::interfaces::ddd::domain::deriveValueObject(tokens);
// }
