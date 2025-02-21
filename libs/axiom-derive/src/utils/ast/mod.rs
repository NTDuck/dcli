use quote::format_ident;
use quote::quote;

pub fn getGenericIdentsFromDeriveInput(ast: &syn::DeriveInput) -> Vec<&syn::Ident> {
    return ast.generics.params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(ty) = param {
                return Some(&ty.ident);
            } else {
                return None;
            }
        })
        .collect();
}

pub fn getWhereClauseWithTraitBoundsFromWhereClauseAndTraitBounds(
    whereClause: Option<&syn::WhereClause>,
    traitBounds: impl Iterator<Item = proc_macro2::TokenStream>
) -> proc_macro2::TokenStream {
    if whereClause.is_some() {
        return quote! {
            #whereClause,
            #( #traitBounds, )*
        };
    } else {
        return quote! {
            where
                #( #traitBounds, )*
        };
    }
}

pub fn getFieldIdentsFromNamedFields(fields: &syn::FieldsNamed) -> Vec<syn::Ident> {
    return fields.named
        .iter()
        .filter_map(|field| field.ident.clone())
        .collect();
}

pub fn getFieldsIdentsFromUnnamedFields(fields: &syn::FieldsUnnamed) -> Vec<syn::Ident> {
    const Prefix: &str = "arg";
    const Suffix: &str = "";

    return (0..fields.unnamed.len())
        .map(|index| format_ident!("{Prefix}{index}{Suffix}"))
        .collect();
}

pub fn getFieldIndicesFromUnnamedFields(fields: &syn::FieldsUnnamed) -> Vec<syn::Index> {
    return (0..fields.unnamed.len())
        .map(syn::Index::from)
        .collect();
}
