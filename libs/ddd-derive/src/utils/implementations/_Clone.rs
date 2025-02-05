use crate::utils::implementation;
use crate::utils::IntermediateTokenStream;

pub fn generate_Clone_impl(ident: &syn::Ident, generics: &syn::Generics, field_idents: &[syn::Member]) -> IntermediateTokenStream {
    return implementation! {
        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                return Self {
                    #(#field_idents: self.#field_idents.clone()),*
                };
            }
        }
    };
}
