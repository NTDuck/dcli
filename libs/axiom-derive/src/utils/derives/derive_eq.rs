use quote::quote;

use crate::utils::ast::*;

pub fn derive_eq(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let (impl_generics, type_generics, _) = ast.generics.split_for_impl();

    let where_clause_with_eq_bounds = generate_where_clause_with_eq_bounds_from_derive_input(ast);

    quote! {
        impl #impl_generics Eq for #ident #type_generics #where_clause_with_eq_bounds {}
    }
}

fn generate_where_clause_with_eq_bounds_from_derive_input(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: Eq
            }
        },
        ast,
    )
}
