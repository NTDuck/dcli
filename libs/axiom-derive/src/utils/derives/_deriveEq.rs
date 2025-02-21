use quote::quote;

use crate::utils::ast::*;

pub fn deriveEq(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let (implGenerics, typeGenerics, _) = ast.generics.split_for_impl();

    let whereClauseWithEqBounds = getWhereClauseWithEqBoundsFromDeriveInput(ast);

    return quote! {
        impl #implGenerics Eq for #ident #typeGenerics #whereClauseWithEqBounds {}
    };
}

fn getWhereClauseWithEqBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let (_, _, whereClause) = ast.generics.split_for_impl();

    let genericIdents = getGenericIdentsFromDeriveInput(ast);
    let traitBounds = genericIdents
        .iter()
        .map(|ident| quote! {
            #ident: Eq
        });
    
    return getWhereClauseWithTraitBoundsFromWhereClauseAndTraitBounds(whereClause, traitBounds);
}
