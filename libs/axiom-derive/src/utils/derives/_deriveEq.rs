use quote::quote;

use crate::utils::ast::*;

pub fn deriveEq(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let (implGenerics, typeGenerics, _) = ast.generics.split_for_impl();

    let whereClauseWithEqBounds = generateWhereClauseWithEqBoundsFromDeriveInput(ast);

    return quote! {
        impl #implGenerics Eq for #ident #typeGenerics #whereClauseWithEqBounds {}
    };
}

fn generateWhereClauseWithEqBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: Eq
        },
        ast,
    );
}
