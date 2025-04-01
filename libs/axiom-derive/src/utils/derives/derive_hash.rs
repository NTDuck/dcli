use quote::quote;

use crate::utils::ast::*;

pub fn derive_hash_for_struct(
    ast: &syn::DeriveInput,
    data: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    match fields {
        syn::Fields::Named(fields) =>
            derive_hash_for_ordinary_struct(ast, fields),
        syn::Fields::Unnamed(fields) =>
            derive_hash_for_tuple_struct(ast, fields),
        syn::Fields::Unit =>
            derive_hash_for_unit_struct(ast),
    }
}

fn derive_hash_for_ordinary_struct(
    ast: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) =
        ast.generics.split_for_impl();
    let struct_where_clause_with_hash_bounds =
        generate_where_clause_with_hash_bounds_from_derive_input(ast);

    let field_idents = get_field_idents_from_named_fields(fields);

    quote! {
        impl #struct_impl_generics std::hash::Hash for #struct_ident #struct_type_generics #struct_where_clause_with_hash_bounds {
            fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
                #( self.#field_idents.hash(state); )*
            }
        }
    }
}

fn derive_hash_for_tuple_struct(
    ast: &syn::DeriveInput,
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) =
        ast.generics.split_for_impl();
    let struct_where_clause_with_hash_bounds =
        generate_where_clause_with_hash_bounds_from_derive_input(ast);

    let field_indices = get_field_indices_from_unnamed_fields(fields);

    quote! {
        impl #struct_impl_generics std::hash::Hash for #struct_ident #struct_type_generics #struct_where_clause_with_hash_bounds {
            fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
                #( self.#field_indices.hash(state); )*
            }
        }
    }
}

fn derive_hash_for_unit_struct(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) =
        ast.generics.split_for_impl();
    let struct_where_clause_with_hash_bounds =
        generate_where_clause_with_hash_bounds_from_derive_input(ast);

    quote! {
        impl #struct_impl_generics std::hash::Hash for #struct_ident #struct_type_generics #struct_where_clause_with_hash_bounds {
            fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {}
        }
    }
}

pub fn derive_hash_for_enum(
    ast: &syn::DeriveInput,
    data: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enum_ident = &ast.ident;
    let (enum_impl_generics, enum_type_generics, _) =
        ast.generics.split_for_impl();
    let enum_where_clause_with_hash_bounds =
        generate_where_clause_with_hash_bounds_from_derive_input(ast);

    let variant_hash_impls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) =>
                derive_hash_for_struct_variant(variant, fields),
            syn::Fields::Unnamed(fields) =>
                derive_hash_for_tuple_variant(variant, fields),
            syn::Fields::Unit =>
                derive_hash_for_unit_variant(variant),
        })
        .collect::<Vec<_>>();

    if variants.is_empty() {
        quote! {
            impl #enum_impl_generics std::hash::Hash for #enum_ident #enum_type_generics #enum_where_clause_with_hash_bounds {
                fn hash<Hasher: std::hash::Hasher>(&self, _: &mut Hasher) {
                    match *self {}
                }
            }
        }
    } else {
        quote! {
            impl #enum_impl_generics std::hash::Hash for #enum_ident #enum_type_generics #enum_where_clause_with_hash_bounds {
                fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
                    core::mem::discriminant(self).hash(state);

                    match self {
                        #( #variant_hash_impls, )*
                    }
                }
            }
        }
    }
}

fn derive_hash_for_struct_variant(
    variant: &syn::Variant,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    let field_idents =
        fields.named.iter().map(|field| &field.ident).collect::<Vec<_>>();

    quote! {
        Self::#variant_ident { #( #field_idents, )* } => {
            #( #field_idents.hash(state); )*
        }
    }
}

fn derive_hash_for_tuple_variant(
    variant: &syn::Variant,
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    let field_idents = get_field_idents_from_unnamed_fields(fields);

    quote! {
        Self::#variant_ident(#( #field_idents, )*) => {
            #( #field_idents.hash(state); )*
        }
    }
}

fn derive_hash_for_unit_variant(
    variant: &syn::Variant,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

    quote! {
        Self::#variant_ident => {}
    }
}

fn generate_where_clause_with_hash_bounds_from_derive_input(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: std::hash::Hash
            }
        },
        ast,
    )
}
