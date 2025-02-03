use darling::FromDeriveInput;

use crate::utils::*;

#[derive(darling::FromDeriveInput)]
#[darling(supports(any))]
pub struct Payload {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: darling::ast::Data<syn::Variant, syn::Field>,
}

impl TryFrom<AbstractSyntaxTree> for Payload {
    type Error = darling::Error;

    fn try_from(ast: AbstractSyntaxTree) -> Result<Self, Self::Error> {
        return Self::from_derive_input(&ast);
    }
}
