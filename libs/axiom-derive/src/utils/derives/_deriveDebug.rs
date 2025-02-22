use quote::quote;

use crate::utils::ast::*;

pub fn deriveDebugForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => deriveDebugForOrdinaryStruct(ast, fields),
        syn::Fields::Unnamed(fields) => deriveDebugForTupleStruct(ast, fields),
        syn::Fields::Unit => deriveDebugForUnitStruct(ast),
    };
}

fn deriveDebugForOrdinaryStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithDebugBounds = generateWhereClauseWithDebugBoundsFromDeriveInput(ast);

    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    return quote! {
        impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClauseWithDebugBounds {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#structIdent))
                    #( .field(stringify!(#fieldIdents), &self.#fieldIdents) )*
                    .finish();
            }
        }
    };
}

fn deriveDebugForTupleStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithDebugBounds = generateWhereClauseWithDebugBoundsFromDeriveInput(ast);

    let fieldIndices = getFieldIndicesFromUnnamedFields(fields);

    return quote! {
        impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClauseWithDebugBounds {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_tuple(stringify!(#structIdent))
                    #( .field(&self.#fieldIndices) )*
                    .finish();
            }
        }
    };
}

fn deriveDebugForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithDebugBounds = generateWhereClauseWithDebugBoundsFromDeriveInput(ast);

    return quote! {
        impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClauseWithDebugBounds {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#structIdent))
                    .finish();
            }
        }
    };
}

pub fn deriveDebugForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();
    let enumWhereClauseWithDebugBounds = generateWhereClauseWithDebugBoundsFromDeriveInput(ast);

    let variantDebugImpls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => deriveDebugForStructVariant(ast, variant, fields),
            syn::Fields::Unnamed(fields) => deriveDebugForTupleVariant(ast, variant, fields),
            syn::Fields::Unit => deriveDebugForUnitVariant(variant),
        })
        .collect::<Vec<_>>();

    if variants.is_empty() {
        return quote! {
            impl #enumImplGenerics std::fmt::Debug for #enumIdent #enumTypeGenerics #enumWhereClauseWithDebugBounds {
                fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match *self {}
                }
            }
        };
    } else {
        return quote! {
            impl #enumImplGenerics std::fmt::Debug for #enumIdent #enumTypeGenerics #enumWhereClauseWithDebugBounds {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    return match self {
                        #( #variantDebugImpls, )*
                    };
                }
            }
        };
    }
}

fn deriveDebugForStructVariant(ast: &syn::DeriveInput, variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
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

fn deriveDebugForTupleVariant(ast: &syn::DeriveInput, variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let variantIdent = &variant.ident;
    let fieldIdents = getFormattedFieldsIdentsFromUnnamedFields(fields);

    return quote! {
        Self::#variantIdent(#( #fieldIdents, )*) => formatter
            .debug_tuple(stringify!(#enumIdent))
            #( .field(#fieldIdents) )*
            .finish()
    };
}

fn deriveDebugForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;

    return quote! {
        Self::#variantIdent => write!(formatter, stringify!(#variantIdent))
    };
}

fn generateWhereClauseWithDebugBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: std::fmt::Debug
        },
        ast,
    );
}
