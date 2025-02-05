use crate::utils::tokens;
use crate::utils::TokenStream;

pub fn generate_Clone_impl(ident: &syn::Ident, generics: &syn::Generics, field_idents: &[syn::Member]) -> TokenStream {
    return tokens! {
        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                return Self {
                    #(#field_idents: self.#field_idents.clone()),*
                };
            }
        }
    };
}