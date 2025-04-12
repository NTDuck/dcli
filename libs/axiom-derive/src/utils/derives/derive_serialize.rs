use quote::quote;

use crate::utils::ast::*;

pub fn derive_serialize_for_struct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    match fields {
        syn::Fields::Named(fields) => derive_serialize_for_ordinary_struct(ast, fields),
        syn::Fields::Unnamed(fields) => derive_serialize_for_tuple_struct(ast, fields),
        syn::Fields::Unit => derive_serialize_for_unit_struct(ast),
    }
}

fn derive_serialize_for_ordinary_struct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) = ast.generics.split_for_impl();
    let struct_where_clause_with_serialize_bounds = generate_where_clause_with_serialize_bounds_from_derive_input(ast);

    let field_idents = get_field_idents_from_named_fields(fields);
    let field_count = fields.named.len();

    quote! {
        impl #struct_impl_generics serde::Serialize for #struct_ident #struct_type_generics #struct_where_clause_with_serialize_bounds {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;

                let mut state = serializer.serialize_struct(stringify!(#struct_ident), #field_count)?;
                #( state.serialize_field(stringify!(#field_idents), &self.#field_idents)?; )*
                return state.end();
            }
        }
    }
}

fn derive_serialize_for_tuple_struct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) = ast.generics.split_for_impl();
    let struct_where_clause_with_serialize_bounds = generate_where_clause_with_serialize_bounds_from_derive_input(ast);

    let field_indices = get_field_indices_from_unnamed_fields(fields);
    let field_count = field_indices.len();

    if field_count == 1 {
        quote! {
            impl #struct_impl_generics serde::Serialize for #struct_ident #struct_type_generics #struct_where_clause_with_serialize_bounds {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeTupleStruct;

                    return serializer.serialize_newtype_struct(stringify!(#struct_ident), &self.0);
                }
            }
        }
    } else {
        quote! {
            impl #struct_impl_generics serde::Serialize for #struct_ident #struct_type_generics #struct_where_clause_with_serialize_bounds {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeTupleStruct;

                    let mut state = serializer.serialize_tuple_struct(stringify!(#struct_ident), #field_count)?;
                    #( state.serialize_field(&self.#field_indices)?; )*
                    return state.end();
                }
            }
        }
    }
}

fn derive_serialize_for_unit_struct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) = ast.generics.split_for_impl();
    let struct_where_clause_with_serialize_bounds = generate_where_clause_with_serialize_bounds_from_derive_input(ast);

    quote! {
        impl #struct_impl_generics serde::Serialize for #struct_ident #struct_type_generics #struct_where_clause_with_serialize_bounds {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                return serializer.serialize_unit_struct(stringify!(#struct_ident));
            }
        }
    }
}

pub fn derive_serialize_for_enum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enum_ident = &ast.ident;
    let (enum_impl_generics, enum_type_generics, _) = ast.generics.split_for_impl();
    let enum_where_clause_with_serialize_bounds = generate_where_clause_with_serialize_bounds_from_derive_input(ast);

    let variant_serialize_impls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => derive_serialize_for_struct_variant(ast, variant, fields),
            syn::Fields::Unnamed(fields) => derive_serialize_for_tuple_variant(ast, variant, fields),
            syn::Fields::Unit => derive_serialize_for_unit_variant(ast, variant),
        })
        .collect::<Vec<_>>();

    if variants.is_empty() {
        quote! {
            impl #enum_impl_generics serde::Serialize for #enum_ident #enum_type_generics #enum_where_clause_with_serialize_bounds {
                fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    match *self {}
                }
            }
        }
    } else {
        quote! {
            impl #enum_impl_generics serde::Serialize for #enum_ident #enum_type_generics #enum_where_clause_with_serialize_bounds {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    match self {
                        #( #variant_serialize_impls, )*
                    }
                }
            }
        }
    }
}

fn derive_serialize_for_struct_variant(
    ast: &syn::DeriveInput,
    variant: &syn::Variant,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let enum_ident = &ast.ident;

    let variant_ident = &variant.ident;
    let variant_index = get_variant_index(ast, variant).unwrap();

    let field_idents = get_field_idents_from_named_fields(fields);
    let field_count = fields.named.len();

    quote! {
        Self::#variant_ident { #( #field_idents, )* } => {
            use serde::ser::SerializeStructVariant;

            let mut state = serializer.serialize_struct_variant(
                stringify!(#enum_ident),
                #variant_index,
                stringify!(#variant_ident),
                #field_count,
            )?;
            #( state.serialize_field(stringify!(#field_idents), #field_idents)?; )*
            return state.end();
        }
    }
}

fn derive_serialize_for_tuple_variant(
    ast: &syn::DeriveInput,
    variant: &syn::Variant,
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let enum_ident = &ast.ident;

    let variant_ident = &variant.ident;
    let variant_index = get_variant_index(ast, variant).unwrap();

    let field_idents = get_field_idents_from_unnamed_fields(fields);
    let field_count = fields.unnamed.len();

    if field_count == 1 {
        quote! {
            Self::#variant_ident(arg) => {
                use serde::ser::SerializeTupleVariant;

                return serializer.serialize_newtype_variant(
                    stringify!(#enum_ident),
                    #variant_index,
                    stringify!(#variant_ident),
                    &arg,
                );
            }
        }
    } else {
        quote! {
            Self::#variant_ident(#( #field_idents, )*) => {
                use serde::ser::SerializeTupleVariant;

                let mut state = serializer.serialize_tuple_variant(
                    stringify!(#enum_ident),
                    #variant_index,
                    stringify!(#variant_ident),
                    #field_count,
                )?;
                #( state.serialize_field(#field_idents)?; )*
                return state.end();
            }
        }
    }
}

fn derive_serialize_for_unit_variant(ast: &syn::DeriveInput, variant: &syn::Variant) -> proc_macro2::TokenStream {
    let enum_ident = &ast.ident;

    let variant_ident = &variant.ident;
    let variant_index = get_variant_index(ast, variant).unwrap();

    quote! {
        Self::#variant_ident => serializer.serialize_unit_variant(
            stringify!(#enum_ident),
            #variant_index,
            stringify!(#variant_ident),
        )
    }
}

fn generate_where_clause_with_serialize_bounds_from_derive_input(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: serde::Serialize
            }
        },
        ast,
    )
}

fn get_variant_index(ast: &syn::DeriveInput, variant: &syn::Variant) -> Option<u32> {
    if let syn::Data::Enum(data) = &ast.data {
        data.variants.iter().position(|v| v.ident == variant.ident).map(|index| index as u32)
    } else {
        None
    }
}
