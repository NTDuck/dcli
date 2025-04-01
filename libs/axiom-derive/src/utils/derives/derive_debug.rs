use quote::quote;

use crate::utils::ast::*;

pub fn derive_debug_for_struct(
    ast: &syn::DeriveInput,
    data: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    match fields {
        syn::Fields::Named(fields) =>
            derive_debug_for_ordinary_struct(ast, fields),
        syn::Fields::Unnamed(fields) =>
            derive_debug_for_tuple_struct(ast, fields),
        syn::Fields::Unit => derive_debug_for_unit_struct(ast),
    }
}

fn derive_debug_for_ordinary_struct(
    ast: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) =
        ast.generics.split_for_impl();
    let struct_where_clause_with_debug_bounds =
        generate_where_clause_with_debug_bounds_from_derive_input(ast);

    let field_idents = get_field_idents_from_named_fields(fields);

    quote! {
        impl #struct_impl_generics std::fmt::Debug for #struct_ident #struct_type_generics #struct_where_clause_with_debug_bounds {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#struct_ident))
                    #( .field(stringify!(#field_idents), &self.#field_idents) )*
                    .finish();
            }
        }
    }
}

fn derive_debug_for_tuple_struct(
    ast: &syn::DeriveInput,
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) =
        ast.generics.split_for_impl();
    let struct_where_clause_with_debug_bounds =
        generate_where_clause_with_debug_bounds_from_derive_input(ast);

    let field_indices = get_field_indices_from_unnamed_fields(fields);

    quote! {
        impl #struct_impl_generics std::fmt::Debug for #struct_ident #struct_type_generics #struct_where_clause_with_debug_bounds {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_tuple(stringify!(#struct_ident))
                    #( .field(&self.#field_indices) )*
                    .finish();
            }
        }
    }
}

fn derive_debug_for_unit_struct(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) =
        ast.generics.split_for_impl();
    let struct_where_clause_with_debug_bounds =
        generate_where_clause_with_debug_bounds_from_derive_input(ast);

    quote! {
        impl #struct_impl_generics std::fmt::Debug for #struct_ident #struct_type_generics #struct_where_clause_with_debug_bounds {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#struct_ident))
                    .finish();
            }
        }
    }
}

pub fn derive_debug_for_enum(
    ast: &syn::DeriveInput,
    data: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enum_ident = &ast.ident;
    let (enum_impl_generics, enum_type_generics, _) =
        ast.generics.split_for_impl();
    let enum_where_clause_with_debug_bounds =
        generate_where_clause_with_debug_bounds_from_derive_input(ast);

    let variant_debug_impls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) =>
                derive_debug_for_struct_variant(ast, variant, fields),
            syn::Fields::Unnamed(fields) =>
                derive_debug_for_tuple_variant(ast, variant, fields),
            syn::Fields::Unit => derive_debug_for_unit_variant(variant),
        })
        .collect::<Vec<_>>();

    if variants.is_empty() {
        quote! {
            impl #enum_impl_generics std::fmt::Debug for #enum_ident #enum_type_generics #enum_where_clause_with_debug_bounds {
                fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match *self {}
                }
            }
        }
    } else {
        quote! {
            impl #enum_impl_generics std::fmt::Debug for #enum_ident #enum_type_generics #enum_where_clause_with_debug_bounds {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    return match self {
                        #( #variant_debug_impls, )*
                    };
                }
            }
        }
    }
}

fn derive_debug_for_struct_variant(
    ast: &syn::DeriveInput,
    variant: &syn::Variant,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let enum_ident = &ast.ident;
    let variant_ident = &variant.ident;
    let field_idents = get_field_idents_from_named_fields(fields);

    quote! {
        Self::#variant_ident { #( #field_idents, )* } => formatter
            .debug_struct(stringify!(#enum_ident))
            #( .field(stringify!(#field_idents), #field_idents) )*
            .finish()
    }
}

fn derive_debug_for_tuple_variant(
    ast: &syn::DeriveInput,
    variant: &syn::Variant,
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let enum_ident = &ast.ident;
    let variant_ident = &variant.ident;
    let field_idents = get_field_idents_from_unnamed_fields(fields);

    quote! {
        Self::#variant_ident(#( #field_idents, )*) => formatter
            .debug_tuple(stringify!(#enum_ident))
            #( .field(#field_idents) )*
            .finish()
    }
}

fn derive_debug_for_unit_variant(
    variant: &syn::Variant,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

    quote! {
        Self::#variant_ident => write!(formatter, stringify!(#variant_ident))
    }
}

fn generate_where_clause_with_debug_bounds_from_derive_input(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: std::fmt::Debug
            }
        },
        ast,
    )
}
