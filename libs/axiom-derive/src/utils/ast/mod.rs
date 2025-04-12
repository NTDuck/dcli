use quote::format_ident;

pub fn generate_where_clause_with_trait_bounds_from_derive_input(
    trait_bounds: impl Fn(&syn::Ident) -> proc_macro2::TokenStream,
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let (_, _, where_clause) = ast.generics.split_for_impl();

    let trait_bounds = ast
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(ty) = param {
                Some(&ty.ident)
            } else {
                None
            }
        })
        .map(trait_bounds);

    if where_clause.is_some() {
        quote::quote! {
            #where_clause,
            #( #trait_bounds, )*
        }
    } else {
        quote::quote! {
            where
                #( #trait_bounds, )*
        }
    }
}

pub fn get_field_idents_from_named_fields(fields: &syn::FieldsNamed) -> Vec<syn::Ident> {
    fields.named.iter().filter_map(|field| field.ident.clone()).collect()
}

pub fn get_field_idents_from_unnamed_fields(fields: &syn::FieldsUnnamed) -> Vec<syn::Ident> {
    return (0..fields.unnamed.len()).map(format_field_index).collect();

    fn format_field_index(field_index: usize) -> syn::Ident {
        format_ident!("arg{field_index}")
    }
}

pub fn get_field_indices_from_unnamed_fields(fields: &syn::FieldsUnnamed) -> Vec<syn::Index> {
    (0..fields.unnamed.len()).map(syn::Index::from).collect()
}

pub fn get_field_types_from_named_fields(fields: &syn::FieldsNamed) -> Vec<&syn::Type> {
    fields.named.iter().map(|field| &field.ty).collect()
}

pub fn get_field_types_from_unnamed_fields(fields: &syn::FieldsUnnamed) -> Vec<&syn::Type> {
    fields.unnamed.iter().map(|field| &field.ty).collect()
}

pub fn convert_ident_to_snake_case(ident: &syn::Ident) -> syn::Ident {
    use heck::ToSnakeCase;

    format_ident!("{}", ident.to_string().to_snake_case())
}

pub const SEGMENT_SEPARATOR: &str = "::";
