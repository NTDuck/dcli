use quote::format_ident;
use quote::quote;

use crate::utils::ast::*;

pub fn derivePartialEqForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => derivePartialEqForOrdinaryStruct(ast, fields),
        syn::Fields::Unnamed(fields) => derivePartialEqForTupleStruct(ast, fields),
        syn::Fields::Unit => derivePartialEqForUnitStruct(ast),
    };
}

fn derivePartialEqForOrdinaryStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithPartialEqBounds = generateWhereClauseWithPartialEqBoundsFromDeriveInput(ast);

    let fieldIdents = getFieldIdentsFromNamedFields(fields);

    return quote! {
        impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics
        #structWhereClauseWithPartialEqBounds {
            fn eq(&self, other: &Self) -> bool {
                return #( self.#fieldIdents == other.#fieldIdents && )* true;
            }
        }
    };
}

fn derivePartialEqForTupleStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithPartialEqBounds = generateWhereClauseWithPartialEqBoundsFromDeriveInput(ast);

    let fieldIndices = getFieldIndicesFromUnnamedFields(fields);

    return quote! {
        impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClauseWithPartialEqBounds {
            fn eq(&self, other: &Self) -> bool {
                return #( self.#fieldIndices == other.#fieldIndices && )* true;
            }
        }
    };
}

fn derivePartialEqForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();
    let structWhereClauseWithPartialEqBounds = generateWhereClauseWithPartialEqBoundsFromDeriveInput(ast);

    return quote! {
        impl #structImplGenerics PartialEq for #structIdent #structTypeGenerics #structWhereClauseWithPartialEqBounds {
            fn eq(&self, other: &Self) -> bool {
                return true;
            }
        }
    };
}

pub fn derivePartialEqForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();
    let enumWhereClauseWithPartialEqBounds = generateWhereClauseWithPartialEqBoundsFromDeriveInput(ast);

    let variantPartialEqImpls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => derivePartialEqForStructVariant(variant, fields),
            syn::Fields::Unnamed(fields) => derivePartialEqForTupleVariant(variant, fields),
            syn::Fields::Unit => derivePartialEqForUnitVariant(variant),
        })
        .collect::<Vec<_>>();

    if variants.is_empty() {
        return quote! {
            impl #enumImplGenerics PartialEq for #enumIdent #enumTypeGenerics #enumWhereClauseWithPartialEqBounds {
                fn eq(&self, _: &Self) -> bool {
                    match *self {}
                }
            }
        };
    } else {
        return quote! {
            impl #enumImplGenerics PartialEq for #enumIdent #enumTypeGenerics #enumWhereClauseWithPartialEqBounds {
                fn eq(&self, other: &Self) -> bool {
                    return match (self, other) {
                        #( #variantPartialEqImpls, )*
                        _ => false,
                    };
                }
            }
        };
    }
}

fn derivePartialEqForStructVariant(variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;

    let fieldIdents = getFieldIdentsFromNamedFields(fields);
    let selfFieldIdents = getIdentsWithSelfPrefixed(&fieldIdents);
    let otherFieldIdents = getIdentsWithOtherPrefixed(&fieldIdents);

    return quote! {
        (
            Self::#variantIdent { #( #fieldIdents: #selfFieldIdents, )* },
            Self::#variantIdent { #( #fieldIdents: #otherFieldIdents, )* },
        ) => {
            return #( #selfFieldIdents == #otherFieldIdents && )* true;
        }
    };
}

fn derivePartialEqForTupleVariant(variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;

    let fieldIdents = getFormattedFieldsIdentsFromUnnamedFields(fields);
    let selfFieldIdents = getIdentsWithSelfPrefixed(&fieldIdents);
    let otherFieldIdents = getIdentsWithOtherPrefixed(&fieldIdents);

    return quote! {
        (
            Self::#variantIdent(#( #selfFieldIdents, )*),
            Self::#variantIdent(#( #otherFieldIdents, )*)
        ) => {
            return #( #selfFieldIdents == #otherFieldIdents && )* true;
        }
    };
}

fn derivePartialEqForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;

    return quote! {
        (Self::#variantIdent, Self::#variantIdent) => true
    };
}

fn generateWhereClauseWithPartialEqBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: PartialEq
        },
        ast,
    );
}

fn getIdentsWithSelfPrefixed(idents: &Vec<syn::Ident>) -> Vec<syn::Ident> {
    return idents
        .iter()
        .map(|ident| format_ident!("self{}", ident))
        .map(|ident| convertIdentToCamelCase(&ident))
        .collect();
}

fn getIdentsWithOtherPrefixed(idents: &Vec<syn::Ident>) -> Vec<syn::Ident> {
    return idents
        .iter()
        .map(|ident| format_ident!("other{}", ident))
        .map(|ident| convertIdentToCamelCase(&ident))
        .collect();
}
