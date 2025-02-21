use quote::quote;

use crate::utils::ast::*;

pub fn deriveCloneForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => deriveCloneForNamedStruct(ast, fields),
        syn::Fields::Unnamed(fields) => deriveCloneForTupleStruct(ast, fields),
        syn::Fields::Unit => deriveCloneForUnitStruct(ast),
    };
}

fn deriveCloneForNamedStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (implGenerics, typeGenerics, _) = ast.generics.split_for_impl();
    let cloneBoundedWhereClause = getCloneBoundedWhereClauseFromDeriveInput(ast);
    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    return quote! {
        impl #implGenerics Clone for #structIdent #typeGenerics #cloneBoundedWhereClause {
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
    let (implGenerics, typeGenerics, _) = ast.generics.split_for_impl();
    let cloneBoundedWhereClause = getCloneBoundedWhereClauseFromDeriveInput(ast);
    let fieldIndices = getFieldIndicesFromUnnamedFields(fields);

    return quote! {
        impl #implGenerics Clone for #structIdent #typeGenerics #cloneBoundedWhereClause {
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
    let (implGenerics, typeGenerics, _) = ast.generics.split_for_impl();
    let cloneBoundedWhereClause = getCloneBoundedWhereClauseFromDeriveInput(ast);

    return quote! {
        impl #implGenerics Clone for #structIdent #typeGenerics #cloneBoundedWhereClause {
            fn clone(&self) -> Self {
                return Self;
            }
        }
    };
}

pub fn deriveCloneForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enumIdent = &ast.ident;
    let (implGenerics, typeGenerics, _) = ast.generics.split_for_impl();
    let cloneBoundedWhereClause = getCloneBoundedWhereClauseFromDeriveInput(ast);

    let variantCloneImpls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => deriveCloneForStructVariant(variant, fields),
            syn::Fields::Unnamed(fields) => deriveCloneForTupleVariant(variant, fields),
            syn::Fields::Unit => deriveCloneForUnitVariant(variant),
        })
        .collect::<Vec<_>>();

    return quote! {
        impl #implGenerics Clone for #enumIdent #typeGenerics #cloneBoundedWhereClause {
            fn clone(&self) -> Self {
                return match self {
                    #( #variantCloneImpls, )*
                };
            }
        }
    }
}

pub fn deriveCloneForStructVariant(variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    return quote! {
        Self::#variantIdent { #( #fieldIdents, )* } => 
            Self::#variantIdent { #( #fieldIdents: #fieldIdents.clone(), )* }
    };
}

pub fn deriveCloneForTupleVariant(variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let fieldIdents = getFieldsIdentsFromUnnamedFields(fields);

    return quote! {
        Self::#variantIdent(#( #fieldIdents, )*) => 
            Self::#variantIdent(#( #fieldIdents.clone(), )*)
    };
}

pub fn deriveCloneForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;

    return quote! {
        Self::#variantIdent => Self::#variantIdent
    };
}

pub fn getCloneBoundedWhereClauseFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let genericIdents = getGenericIdentsFromDeriveInput(ast);
    let bounds = genericIdents
        .iter()
        .map(|ident| quote! {
            #ident: Clone
        });

    let (_, _, whereClause) = ast.generics.split_for_impl();
    
    return getBoundedWhereClauseFromBoundsAndWhereClause(bounds, whereClause);
}
