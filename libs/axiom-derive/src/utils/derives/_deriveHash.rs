use quote::quote;

use crate::utils::ast::*;

pub fn deriveHashForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => deriveHashForOrdinaryStruct(ast, fields),
        syn::Fields::Unnamed(fields) => deriveHashForTupleStruct(ast, fields),
        syn::Fields::Unit => deriveHashForUnitStruct(ast),
    };
}

fn deriveHashForOrdinaryStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithHashBounds = generateWhereClauseWithHashBoundsFromDeriveInput(ast);

    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    return quote! {
        impl #structImplGenerics std::hash::Hash for #structIdent #structTypeGenerics #structWhereClauseWithHashBounds {
            fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
                #( self.#fieldIdents.hash(state); )*
            }
        }
    };
}

fn deriveHashForTupleStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithHashBounds = generateWhereClauseWithHashBoundsFromDeriveInput(ast);

    let fieldIndices = getFieldIndicesFromUnnamedFields(fields);

    return quote! {
        impl #structImplGenerics std::hash::Hash for #structIdent #structTypeGenerics #structWhereClauseWithHashBounds {
            fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
                #( self.#fieldIndices.hash(state); )*
            }
        }
    };
}

fn deriveHashForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithHashBounds = generateWhereClauseWithHashBoundsFromDeriveInput(ast);

    return quote! {
        impl #structImplGenerics std::hash::Hash for #structIdent #structTypeGenerics #structWhereClauseWithHashBounds {
            fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {}
        }
    };
}

pub fn deriveHashForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();
    let enumWhereClauseWithHashBounds = generateWhereClauseWithHashBoundsFromDeriveInput(ast);

    let variantHashImpls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => deriveHashForStructVariant(variant, fields),
            syn::Fields::Unnamed(fields) => deriveHashForTupleVariant(variant, fields),
            syn::Fields::Unit => deriveHashForUnitVariant(variant),
        })
        .collect::<Vec<_>>();

    if variants.is_empty() {
        return quote! {
            impl #enumImplGenerics std::hash::Hash for #enumIdent #enumTypeGenerics #enumWhereClauseWithHashBounds {
                fn hash<Hasher: std::hash::Hasher>(&self, _: &mut Hasher) {
                    match *self {}
                }
            }
        };
    } else {
        return quote! {
            impl #enumImplGenerics std::hash::Hash for #enumIdent #enumTypeGenerics #enumWhereClauseWithHashBounds {
                fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
                    core::mem::discriminant(self).hash(state);
    
                    match self {
                        #( #variantHashImpls, )*
                    }
                }
            }
        };
    }
}

fn deriveHashForStructVariant(variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let fieldIdents = fields.named
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    return quote! {
        Self::#variantIdent { #( #fieldIdents, )* } => {
            #( #fieldIdents.hash(state); )*
        }
    };
}

fn deriveHashForTupleVariant(variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let fieldIdents = getFormattedFieldsIdentsFromUnnamedFields(fields);

    return quote! {
        Self::#variantIdent(#( #fieldIdents, )*) => {
            #( #fieldIdents.hash(state); )*
        }
    };
}

fn deriveHashForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    
    return quote! {
        Self::#variantIdent => {}
    };
}

fn generateWhereClauseWithHashBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: std::hash::Hash
        },
        ast,
    );
}
