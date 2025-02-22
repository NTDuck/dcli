use quote::quote;

use crate::utils::ast::*;

pub fn deriveCloneForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => deriveCloneForOrdinaryStruct(ast, fields),
        syn::Fields::Unnamed(fields) => deriveCloneForTupleStruct(ast, fields),
        syn::Fields::Unit => deriveCloneForUnitStruct(ast),
    };
}

fn deriveCloneForOrdinaryStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithCloneBounds = generateWhereClauseWithCloneBoundsFromDeriveInput(ast);

    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    return quote! {
        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClauseWithCloneBounds {
            fn clone(&self) -> Self {
                return Self {
                    #( #fieldIdents: self.#fieldIdents.clone(), )*
                };
            }
        }
    };
}

fn deriveCloneForTupleStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithCloneBounds = generateWhereClauseWithCloneBoundsFromDeriveInput(ast);

    let fieldIndices = getFieldIndicesFromUnnamedFields(fields);

    return quote! {
        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClauseWithCloneBounds {
            fn clone(&self) -> Self {
                return Self(
                    #( self.#fieldIndices.clone(), )*
                );
            }
        }
    };
}

fn deriveCloneForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithCloneBounds = generateWhereClauseWithCloneBoundsFromDeriveInput(ast);

    return quote! {
        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClauseWithCloneBounds {
            fn clone(&self) -> Self {
                return Self;
            }
        }
    };
}

pub fn deriveCloneForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();
    let enumWhereClauseWithCloneBounds = generateWhereClauseWithCloneBoundsFromDeriveInput(ast);

    let variantCloneImpls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => deriveCloneForStructVariant(variant, fields),
            syn::Fields::Unnamed(fields) => deriveCloneForTupleVariant(variant, fields),
            syn::Fields::Unit => deriveCloneForUnitVariant(variant),
        })
        .collect::<Vec<_>>();

    if variants.is_empty() {
        return quote! {
            impl #enumImplGenerics Clone for #enumIdent #enumTypeGenerics #enumWhereClauseWithCloneBounds {
                fn clone(&self) -> Self {
                    match *self {}
                }
            }
        };
    } else {
        return quote! {
            impl #enumImplGenerics Clone for #enumIdent #enumTypeGenerics #enumWhereClauseWithCloneBounds {
                fn clone(&self) -> Self {
                    return match self {
                        #( #variantCloneImpls, )*
                    };
                }
            }
        };
    }
}

fn deriveCloneForStructVariant(variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    return quote! {
        Self::#variantIdent { #( #fieldIdents, )* } => 
            Self::#variantIdent { #( #fieldIdents: #fieldIdents.clone(), )* }
    };
}

fn deriveCloneForTupleVariant(variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let fieldIdents = getFormattedFieldsIdentsFromUnnamedFields(fields);

    return quote! {
        Self::#variantIdent(#( #fieldIdents, )*) => 
            Self::#variantIdent(#( #fieldIdents.clone(), )*)
    };
}

fn deriveCloneForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;

    return quote! {
        Self::#variantIdent => Self::#variantIdent
    };
}

fn generateWhereClauseWithCloneBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: Clone
        },
        ast,
    );
}