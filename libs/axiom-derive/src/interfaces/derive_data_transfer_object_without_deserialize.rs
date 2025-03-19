use quote::quote;

use crate::utils::ast::*;
use crate::utils::derives::*;

pub fn derive_data_transfer_object_without_deserialize(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => derive_for_struct(&ast, data),
        syn::Data::Enum(data) => derive_for_enum(&ast, data),
        _ => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn derive_for_struct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) = ast.generics.split_for_impl();

    let struct_where_clause_with_data_transfer_object_bounds = generate_where_clause_with_data_transfer_object_bounds_from_derive_input(ast);
    
    let struct_debug_impl = derive_debug_for_struct(ast, data);
    let struct_clone_impl = derive_clone_for_struct(ast, data);
    let struct_serialize_impl = derive_serialize_for_struct(ast, data);

    return quote! {
        impl #struct_impl_generics axiom::interfaces::DataTransferObject for #struct_ident #struct_type_generics #struct_where_clause_with_data_transfer_object_bounds {}

        #struct_debug_impl
        #struct_clone_impl
        #struct_serialize_impl
    };
}

fn derive_for_enum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let enum_ident = &ast.ident;
    let (enum_impl_generics, enum_type_generics, _) = ast.generics.split_for_impl();

    let enum_where_clause_with_data_transfer_object_bounds = generate_where_clause_with_data_transfer_object_bounds_from_derive_input(ast);

    let enum_debug_impl = derive_debug_for_enum(ast, data);
    let enum_clone_impl = derive_clone_for_enum(ast, data);
    let enum_serialize_impl = derive_serialize_for_enum(ast, data);

    return quote! {
        impl #enum_impl_generics axiom::interfaces::DataTransferObject for #enum_ident #enum_type_generics #enum_where_clause_with_data_transfer_object_bounds {}

        #enum_debug_impl
        #enum_clone_impl
        #enum_serialize_impl
    };
}

fn generate_where_clause_with_data_transfer_object_bounds_from_derive_input(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generate_where_clause_with_trait_bounds_from_derive_input(
        |T| quote! {
            #T: axiom::interfaces::DataTransferObject
        },
        ast,
    );
}
