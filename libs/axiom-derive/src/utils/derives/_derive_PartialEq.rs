use quote::format_ident;
use quote::quote;

use crate::utils::ast::*;

pub fn derive_partial_eq_for_struct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => derive_partial_eq_for_ordinary_struct(ast, fields),
        syn::Fields::Unnamed(fields) => derive_partial_eq_for_tuple_struct(ast, fields),
        syn::Fields::Unit => derive_partial_eq_for_unit_struct(ast),
    };
}

fn derive_partial_eq_for_ordinary_struct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) = ast.generics.split_for_impl();
    let struct_where_clause_with_partial_eq_bounds = generate_where_clause_with_partial_eq_bounds_from_derive_input(ast);

    let field_idents = get_field_idents_from_named_fields(fields);

    return quote! {
        impl #struct_impl_generics PartialEq for #struct_ident #struct_type_generics
        #struct_where_clause_with_partial_eq_bounds {
            fn eq(&self, other: &Self) -> bool {
                return #( self.#field_idents == other.#field_idents && )* true;
            }
        }
    };
}

fn derive_partial_eq_for_tuple_struct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) = ast.generics.split_for_impl();
    let struct_where_clause_with_partial_eq_bounds = generate_where_clause_with_partial_eq_bounds_from_derive_input(ast);

    let field_indices = get_field_indices_from_unnamed_fields(fields);

    return quote! {
        impl #struct_impl_generics PartialEq for #struct_ident #struct_type_generics #struct_where_clause_with_partial_eq_bounds {
            fn eq(&self, other: &Self) -> bool {
                return #( self.#field_indices == other.#field_indices && )* true;
            }
        }
    };
}

fn derive_partial_eq_for_unit_struct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) = ast.generics.split_for_impl();
    let struct_where_clause_with_partial_eq_bounds = generate_where_clause_with_partial_eq_bounds_from_derive_input(ast);

    return quote! {
        impl #struct_impl_generics PartialEq for #struct_ident #struct_type_generics #struct_where_clause_with_partial_eq_bounds {
            fn eq(&self, other: &Self) -> bool {
                return true;
            }
        }
    };
}

pub fn derive_partial_eq_for_enum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enum_ident = &ast.ident;
    let (enum_impl_generics, enum_type_generics, _) = ast.generics.split_for_impl();
    let enum_where_clause_with_partial_eq_bounds = generate_where_clause_with_partial_eq_bounds_from_derive_input(ast);

    let variant_partial_eq_impls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => derive_partial_eq_for_struct_variant(variant, fields),
            syn::Fields::Unnamed(fields) => derive_partial_eq_for_tuple_variant(variant, fields),
            syn::Fields::Unit => derive_partial_eq_for_unit_variant(variant),
        })
        .collect::<Vec<_>>();

    if variants.is_empty() {
        return quote! {
            impl #enum_impl_generics PartialEq for #enum_ident #enum_type_generics #enum_where_clause_with_partial_eq_bounds {
                fn eq(&self, _: &Self) -> bool {
                    match *self {}
                }
            }
        };
    } else {
        return quote! {
            impl #enum_impl_generics PartialEq for #enum_ident #enum_type_generics #enum_where_clause_with_partial_eq_bounds {
                fn eq(&self, other: &Self) -> bool {
                    return match (self, other) {
                        #( #variant_partial_eq_impls, )*
                        _ => false,
                    };
                }
            }
        };
    }
}

fn derive_partial_eq_for_struct_variant(variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

    let field_idents = get_field_idents_from_named_fields(fields);
    let selffield_idents = get_idents_with_self_prefixed(&field_idents);
    let otherfield_idents = get_idents_with_other_prefixed(&field_idents);

    return quote! {
        (
            Self::#variant_ident { #( #field_idents: #selffield_idents, )* },
            Self::#variant_ident { #( #field_idents: #otherfield_idents, )* },
        ) => {
            return #( #selffield_idents == #otherfield_idents && )* true;
        }
    };
}

fn derive_partial_eq_for_tuple_variant(variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

    let field_idents = get_formatted_field_idents_from_unnamed_fields(fields);
    let selffield_idents = get_idents_with_self_prefixed(&field_idents);
    let otherfield_idents = get_idents_with_other_prefixed(&field_idents);

    return quote! {
        (
            Self::#variant_ident(#( #selffield_idents, )*),
            Self::#variant_ident(#( #otherfield_idents, )*)
        ) => {
            return #( #selffield_idents == #otherfield_idents && )* true;
        }
    };
}

fn derive_partial_eq_for_unit_variant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

    return quote! {
        (Self::#variant_ident, Self::#variant_ident) => true
    };
}

fn generate_where_clause_with_partial_eq_bounds_from_derive_input(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generate_where_clause_with_trait_bounds_from_derive_input(
        |T| quote! {
            #T: PartialEq
        },
        ast,
    );
}

fn get_idents_with_self_prefixed(idents: &Vec<syn::Ident>) -> Vec<syn::Ident> {
    return idents
        .iter()
        .map(|ident| format_ident!("self{}", ident))
        .map(|ident| convert_ident_to_camel_case(&ident))
        .collect();
}

fn get_idents_with_other_prefixed(idents: &Vec<syn::Ident>) -> Vec<syn::Ident> {
    return idents
        .iter()
        .map(|ident| format_ident!("other{}", ident))
        .map(|ident| convert_ident_to_camel_case(&ident))
        .collect();
}
