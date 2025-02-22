use quote::format_ident;

pub fn generateWhereClauseWithTraitBoundsFromDeriveInput(
    traitBounds: impl Fn(&syn::Ident) -> proc_macro2::TokenStream,
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let (_, _, whereClause) = ast.generics.split_for_impl();

    let traitBounds = ast.generics.params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(ty) = param {
                return Some(&ty.ident);
            } else {
                return None;
            }
        })
        .map(traitBounds);
    
    if whereClause.is_some() {
        quote::quote! {
            #whereClause,
            #( #traitBounds, )*
        }
    } else {
        quote::quote! {
            where
                #( #traitBounds, )*
        }
    }
}

pub fn getFieldIdentsFromNamedFields(fields: &syn::FieldsNamed) -> Vec<syn::Ident> {
    return fields.named
        .iter()
        .filter_map(|field| field.ident.clone())
        .collect();
}

pub fn getFormattedFieldsIdentsFromUnnamedFields(fields: &syn::FieldsUnnamed) -> Vec<syn::Ident> {
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

pub fn getFieldTypesFromNamedFields(fields: &syn::FieldsNamed) -> Vec<&syn::Type> {
    return fields.named
        .iter()
        .map(|field| &field.ty)
        .collect();
}

pub fn getFieldTypesFromUnnamedFields(fields: &syn::FieldsUnnamed) -> Vec<&syn::Type> {
    return fields.unnamed
        .iter()
        .map(|field| &field.ty)
        .collect();
}

pub fn convertIdentToCamelCase(ident: &syn::Ident) -> syn::Ident {
    use heck::ToLowerCamelCase;

    return format_ident!("{}", ident.to_string().to_lower_camel_case());
}
