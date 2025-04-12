use quote::format_ident;
use quote::quote;

use crate::utils::ast::*;

pub fn derive_newtype(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => derive_for_struct(&ast, data),
        _ => panic!(),
    };

    proc_macro::TokenStream::from(tokens)
}

fn derive_for_struct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    let syn::Fields::Unnamed(fields) = fields else {
        panic!("Newtypes must be a single-fielded tuple struct")
    };

    if fields.unnamed.len() != 1 {
        panic!("Newtypes must be a single-fielded tuple struct")
    };

    let field = fields.unnamed.first().unwrap();

    derive_for_single_fielded_tuple_struct(ast, field)
}

fn derive_for_single_fielded_tuple_struct(ast: &syn::DeriveInput, field: &syn::Field) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, struct_where_clause) = ast.generics.split_for_impl();

    let field_type = &field.ty;
    let field_type_ident = get_field_type_ident_from_field(field);
    let as_method_ident = format_ident!("as_{field_type_ident}");
    let to_method_ident = format_ident!("to_{field_type_ident}");

    quote! {
        impl #struct_impl_generics #struct_ident #struct_type_generics #struct_where_clause {
            fn #as_method_ident(&self) -> &#field_type {
                return &self.0;
            }

            fn #to_method_ident(self) -> #field_type {
                return self.0;
            }
        }

        impl #struct_impl_generics std::ops::Deref for #struct_ident #struct_type_generics #struct_where_clause {
            type Target = #field_type;

            fn deref(&self) -> &Self::Target {
                return &self.0;
            }
        }

        impl #struct_impl_generics std::ops::DerefMut for #struct_ident #struct_type_generics #struct_where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                return &mut self.0;
            }
        }
    }
}

fn get_field_type_ident_from_field(field: &syn::Field) -> syn::Ident {
    use heck::ToSnakeCase;
    use quote::ToTokens;

    let field_type_ident = field
        .ty
        .clone()
        .into_token_stream()
        .to_string()
        .split('<')
        .next()
        .unwrap()
        .split(SEGMENT_SEPARATOR)
        .last()
        .unwrap()
        .to_snake_case();

    format_ident!("{}", field_type_ident)
}
