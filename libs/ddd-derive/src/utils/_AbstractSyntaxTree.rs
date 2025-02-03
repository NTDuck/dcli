use std::ops::Deref;

use crate::utils::TokenStream;

pub struct AbstractSyntaxTree(syn::DeriveInput);

impl Deref for AbstractSyntaxTree {
    type Target = syn::DeriveInput;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl TryFrom<TokenStream> for AbstractSyntaxTree {
    type Error = syn::Error;

    fn try_from(tokens: TokenStream) -> Result<Self, Self::Error> {
        return syn::parse(tokens)
            .map(|input| AbstractSyntaxTree(input));
    }
}
