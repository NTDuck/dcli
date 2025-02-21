use quote::{format_ident, quote};

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

pub fn getBoundedWhereClauseFromBoundsAndWhereClause(bounds: impl Iterator<Item = proc_macro2::TokenStream>, whereClause: Option<&syn::WhereClause>) -> proc_macro2::TokenStream {
    if whereClause.is_some() {
        return quote! {
            #whereClause,
            #( #bounds, )*
        };
    } else {
        return quote! {
            where #( #bounds, )*
        };
    }
}

pub fn getFieldIdentsFromNamedFields(fields: &syn::FieldsNamed) -> Vec<&Option<syn::Ident>> {
    return fields.named
        .iter()
        .map(|field| &field.ident)
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
