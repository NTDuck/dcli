use quote::quote;

use crate::utils::ast::*;

pub fn derive_entity(
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => derive_for_struct(&ast, data),
        _ => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn derive_for_struct(
    ast: &syn::DeriveInput,
    data: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => derive_for_ordinary_struct(ast, fields),
        _ => panic!(),
    };
}

fn derive_for_ordinary_struct(
    ast: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let (struct_impl_generics, struct_type_generics, _) =
        ast.generics.split_for_impl();

    let struct_where_clause_with_entity_bounds =
        generate_where_clause_with_entity_bounds_from_derive_input(ast);
    let struct_where_clause_with_value_object_bounds =
        generate_where_clause_with_value_object_bounds_from_derive_input(ast);
    let struct_where_clause_with_debug_bounds =
        generate_where_clause_with_debug_bounds_from_derive_input(ast);
    let struct_where_clause_with_clone_bounds =
        generate_where_clause_with_clone_bounds_from_derive_input(ast);
    let struct_where_clause_with_partial_eq_bounds =
        generate_where_clause_with_partial_eq_bounds_from_derive_input(ast);
    let struct_where_clause_with_eq_bounds =
        generate_where_clause_with_eq_bounds_from_derive_input(ast);

    let field_idents = get_field_idents_from_named_fields(fields);

    let identifier_field = get_identifier_field_for_ordinary_struct(fields)
        .expect(&format!(
            "Struct `{}` must have one field implementing \
             `axiom::interfaces::ddd::domain::Identifier` and annotated {}",
            struct_ident.to_string(),
            ACCEPTED_ATTRIBUTES
                .iter()
                .map(|attr| format!("`#[axiom(attributes({attr}))]`"))
                .collect::<Vec<_>>()
                .join(", "),
        ));

    let identifier_field_ident = &identifier_field.ident;
    let identifier_field_type = &identifier_field.ty;

    return quote! {
        impl #struct_impl_generics axiom::interfaces::ddd::domain::Entity for #struct_ident #struct_type_generics #struct_where_clause_with_entity_bounds {
            type Identifier = #identifier_field_type;

            #[inline(always)]
            fn get_id(&self) -> &Self::Identifier {
                return &self.#identifier_field_ident;
            }
        }

        impl #struct_impl_generics axiom::interfaces::ddd::domain::ValueObject for #struct_ident #struct_type_generics #struct_where_clause_with_value_object_bounds {}

        impl #struct_impl_generics std::fmt::Debug for #struct_ident #struct_type_generics #struct_where_clause_with_debug_bounds {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#struct_ident))
                    #( .field(stringify!(#field_idents), &self.#field_idents) )*
                    .finish();
            }
        }

        impl #struct_impl_generics Clone for #struct_ident #struct_type_generics #struct_where_clause_with_clone_bounds {
            fn clone(&self) -> Self {
                return Self {
                    #( #field_idents: self.#field_idents.clone(), )*
                };
            }
        }

        impl #struct_impl_generics PartialEq for #struct_ident #struct_type_generics #struct_where_clause_with_partial_eq_bounds {
            fn eq(&self, other: &Self) -> bool {
                use axiom::interfaces::ddd::domain::Entity;

                return self.get_id() == other.get_id();
            }
        }

        impl #struct_impl_generics Eq for #struct_ident #struct_type_generics #struct_where_clause_with_eq_bounds {}
    };
}

fn generate_where_clause_with_entity_bounds_from_derive_input(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    return generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: axiom::interfaces::ddd::domain::Entity
            }
        },
        ast,
    );
}

fn generate_where_clause_with_value_object_bounds_from_derive_input(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    return generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: axiom::interfaces::ddd::domain::ValueObject
            }
        },
        ast,
    );
}

fn generate_where_clause_with_debug_bounds_from_derive_input(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    return generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: std::fmt::Debug
            }
        },
        ast,
    );
}

fn generate_where_clause_with_clone_bounds_from_derive_input(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    return generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: Clone
            }
        },
        ast,
    );
}

fn generate_where_clause_with_partial_eq_bounds_from_derive_input(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    return generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: PartialEq
            }
        },
        ast,
    );
}

fn generate_where_clause_with_eq_bounds_from_derive_input(
    ast: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    return generate_where_clause_with_trait_bounds_from_derive_input(
        |type_ident| {
            quote! {
                #type_ident: Eq
            }
        },
        ast,
    );
}

fn get_identifier_field_for_ordinary_struct(
    fields: &syn::FieldsNamed,
) -> Option<&syn::Field> {
    return fields.named.iter().find(|field| {
        field
            .attrs
            .iter()
            .filter_map(|attr| {
                extract_nested_meta_lists_from_attr(attr, "axiom")
            })
            .flat_map(|meta_list| {
                extract_nested_meta_lists_from_meta_list(
                    &meta_list,
                    "attributes",
                )
            })
            .flat_map(extract_nested_attr_idents_from_meta_list)
            .any(|attr_ident| {
                ACCEPTED_ATTRIBUTES.contains(&attr_ident.as_str())
            })
    });
}

fn extract_nested_meta_lists_from_attr(
    attr: &syn::Attribute,
    expected_attr_ident: &str,
) -> Option<syn::MetaList> {
    if !attr.path().is_ident(expected_attr_ident) {
        return None;
    }

    return attr.meta.require_list().ok().cloned();
}

fn extract_nested_meta_lists_from_meta_list(
    meta_list: &syn::MetaList,
    expected_attr_ident: &str,
) -> Vec<syn::MetaList> {
    return meta_list
        .parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .map(|punctuated| punctuated
            .into_iter()
            .filter_map(|meta| meta
                .path()
                .is_ident(expected_attr_ident)
                .then(|| meta.require_list().ok())
                .flatten()
                .cloned())
            .collect())
        .unwrap_or_default();
}

fn extract_nested_attr_idents_from_meta_list(
    meta_list: syn::MetaList,
) -> Vec<String> {
    return meta_list
        .parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
        .ok()
        .map(|punctuated| punctuated
            .into_iter()
            .map(|path| path.segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join(SEGMENT_SEPARATOR))
            .collect())
        .unwrap_or_default();
}

const ACCEPTED_ATTRIBUTES: [&str; 3] =
    ["Identifier", "ddd::Identifier", "ddd::domain::Identifier"];
