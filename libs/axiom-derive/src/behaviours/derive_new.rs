use quote::format_ident;
use quote::quote;

use crate::utils::ast::*;

pub fn derive_new(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => derive_for_struct(&ast, data),
        syn::Data::Enum(data) => derive_for_enum(&ast, data),
        _ => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn derive_for_struct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => derive_for_ordinary_struct(ast, fields),
        syn::Fields::Unnamed(fields) => derive_for_tuple_struct(ast, fields),
        syn::Fields::Unit => derive_for_unit_struct(ast),
    };
}

fn derive_for_ordinary_struct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, struct_where_clause) = ast.generics.split_for_impl();
    let method_ident = get_method_ident_for_struct();

    let field_idents = get_field_idents_from_named_fields(fields);
    let field_types = get_field_types_from_named_fields(fields);

    return quote! {
        impl #struct_impl_generics #struct_ident #struct_type_generics #struct_where_clause {
            pub fn #method_ident(#( #field_idents: #field_types, )*) -> Self {
                return Self { #( #field_idents, )* };
            }
        }
    };
}

fn derive_for_tuple_struct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, struct_where_clause) = ast.generics.split_for_impl();
    let method_ident = get_method_ident_for_struct();

    let field_idents = get_formatted_field_idents_from_unnamed_fields(fields);
    let field_types = get_field_types_from_unnamed_fields(fields);

    return quote! {
        impl #struct_impl_generics #struct_ident #struct_type_generics #struct_where_clause {
            pub fn #method_ident(#( #field_idents: #field_types, )*) -> Self {
                return Self( #( #field_idents, )* );
            }
        }
    };
}

fn derive_for_unit_struct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, struct_where_clause) = ast.generics.split_for_impl();
    let method_ident = get_method_ident_for_struct();

    return quote! {
        impl #struct_impl_generics #struct_ident #struct_type_generics #struct_where_clause {
            pub const fn #method_ident() -> Self {
                return Self;
            }
        }
    };
}

fn derive_for_enum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let variant_new_methods = variants
        .iter()
        .map(|variant| {
            return match &variant.fields {
                syn::Fields::Named(fields) => derive_for_struct_variant(variant, fields),
                syn::Fields::Unnamed(fields) => derive_for_tuple_variant(variant, fields),
                syn::Fields::Unit => derive_for_unit_variant(variant),
            };
        })
        .collect::<Vec<_>>();

    let enum_ident = &ast.ident;
    let (enum_impl_generics, enum_type_generics, enum_where_clause) = ast.generics.split_for_impl();

    return quote! {
        impl #enum_impl_generics #enum_ident #enum_type_generics #enum_where_clause {
            #( #variant_new_methods )*
        }
    }
}

fn derive_for_struct_variant(variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    let method_ident = get_method_ident_for_enum_from_variant(variant);

    let field_idents = get_field_idents_from_named_fields(fields);
    let field_types = get_field_types_from_named_fields(fields);

    return quote! {
        pub fn #method_ident(#( #field_idents: #field_types, )*) -> Self {
            return Self::#variant_ident { #(#field_idents, )* };
        }
    };
}

fn derive_for_tuple_variant(variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    let method_ident = get_method_ident_for_enum_from_variant(variant);

    let fieldIdents = get_formatted_field_idents_from_unnamed_fields(fields);
    let field_types = get_field_types_from_unnamed_fields(fields);

    return quote! {
        pub fn #method_ident(#( #fieldIdents: #field_types, )*) -> Self {
            return Self::#variant_ident( #( #fieldIdents, )* );
        }
    };
}

fn derive_for_unit_variant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    let method_ident = get_method_ident_for_enum_from_variant(variant);

    return quote! {
        pub fn #method_ident() -> Self {
            return Self::#variant_ident;
        }
    };
}

fn get_method_ident_for_struct() -> syn::Ident {
    return format_ident!("{BASE_METHOD_IDENT}");
}

fn get_method_ident_for_enum_from_variant(variant: &syn::Variant) -> syn::Ident {
    let variant_ident = &variant.ident;

    let unformatted_method_ident = format_ident!("{BASE_METHOD_IDENT}{variant_ident}");
    let formatted_method_ident = convert_ident_to_snake_case(&unformatted_method_ident);

    return formatted_method_ident;
}

const BASE_METHOD_IDENT: &str = "new";
