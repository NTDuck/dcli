// use quote::quote;

// use crate::utils::TokenStream;

// pub fn generate_Clone_impl(ident: &syn::Ident, generics: &syn::Generics, field_idents: &[syn::Member]) -> proc_macro2::TokenStream {
//     return quote! {
//         impl #generics Clone for #ident #generics {
//             fn clone(&self) -> Self {
//                 return Self {
//                     #(#field_idents: self.#field_idents.clone()),*
//                 };
//             }
//         }
//     };
// }