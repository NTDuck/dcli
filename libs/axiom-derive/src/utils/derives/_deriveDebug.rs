use quote::quote;

use crate::utils::ast::*;

pub fn deriveDebugForNamedStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (implGenerics, typeGenerics, _) = ast.generics.split_for_impl();
    let debugBoundedWhereClause = getDebugBoundedWhereClauseFromDeriveInput(ast);
    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    return quote! {
        impl #implGenerics std::fmt::Debug for #structIdent #typeGenerics #debugBoundedWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#structIdent))
                    #( .field(stringify!(#fieldIdents), &self.#fieldIdents) )*
                    .finish();
            }
        }
    };
}

pub fn deriveDebugForTupleStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (implGenerics, typeGenerics, _) = ast.generics.split_for_impl();
    let debugBoundedWhereClause = getDebugBoundedWhereClauseFromDeriveInput(ast);
    let fieldIndices = getFieldIndicesFromUnnamedFields(fields);

    return quote! {
        impl #implGenerics std::fmt::Debug for #structIdent #typeGenerics #debugBoundedWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_tuple(stringify!(#structIdent))
                    #( .field(&self.#fieldIndices) )*
                    .finish();
            }
        }
    };
}

pub fn deriveDebugForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (implGenerics, typeGenerics, _) = ast.generics.split_for_impl();
    let debugBoundedWhereClause = getDebugBoundedWhereClauseFromDeriveInput(ast);

    return quote! {
        impl #implGenerics std::fmt::Debug for #structIdent #typeGenerics #debugBoundedWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#structIdent))
                    .finish();
            }
        }
    };
}

pub fn deriveDebugForStructVariant(ast: &syn::DeriveInput, variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let variantIdent = &variant.ident;
    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    return quote! {
        Self::#variantIdent { #( #fieldIdents, )* } => formatter
            .debug_struct(stringify!(#enumIdent))
            #( .field(stringify!(#fieldIdents), #fieldIdents) )*
            .finish()
    };
}

pub fn deriveDebugForTupleVariant(ast: &syn::DeriveInput, variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let variantIdent = &variant.ident;
    let fieldIdents = getFieldsIdentsFromUnnamedFields(fields);

    return quote! {
        Self::#variantIdent(#( #fieldIdents, )*) => formatter
            .debug_tuple(stringify!(#enumIdent))
            #( .field(#fieldIdents) )*
            .finish()
    };
}

pub fn deriveDebugForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;

    return quote! {
        Self::#variantIdent => write!(formatter, stringify!(#variantIdent))
    };
}

pub fn getDebugBoundedWhereClauseFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let genericIdents = getGenericIdentsFromDeriveInput(ast);
    let bounds = genericIdents
        .iter()
        .map(|ident| quote! {
            #ident: std::fmt::Debug
        });

    let (_, _, whereClause) = ast.generics.split_for_impl();
    
    return getBoundedWhereClauseFromBoundsAndWhereClause(bounds, whereClause);
}
