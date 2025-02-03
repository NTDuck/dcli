use darling::FromDeriveInput;
use quote::quote;

use crate::utils::*;

pub fn derive_value_object(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return TokenStream::from(error.into_compile_error()),
    };

    let payload = match Payload::try_from(ast) {
        Ok(payload) => payload,
        Err(error) => return TokenStream::from(error.write_errors()),
    };

    match payload.data {
        darling::ast::Data::Struct(fields) => {
            return StructPayload {
                ident: payload.ident,
                generics: payload.generics,
                fields,
            }.into();
        },
        darling::ast::Data::Enum(variants) => {
            todo!()
        },
    }
}

#[derive(darling::FromDeriveInput)]
#[darling(supports(struct_any))]
struct Payload {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), syn::Field>,
}

impl TryFrom<AbstractSyntaxTree> for Payload {
    type Error = darling::Error;

    fn try_from(ast: AbstractSyntaxTree) -> Result<Self, Self::Error> {
        return Self::from_derive_input(&ast);
    }
}

struct StructPayload {
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<syn::Field>,
}

impl Into<TokenStream> for StructPayload {
    fn into(self) -> TokenStream {
        return generate_token_stream_for_struct(self);
    }
}

fn generate_token_stream_for_struct(payload: StructPayload) -> TokenStream {
    let StructPayload {
        ident, generics, fields,
    } = payload;

    let field = match is_named_struct(&fields) {
        true => generate_fields_for_named_struct(fields),
        false => generate_fields_for_unnamed_struct(fields),
    };

    return quote! {
        impl #generics ddd::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                Self {
                    #(#field: self.#field.clone(),)*
                }
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                true #( && self.#field == other.#field)*
            }
        }

        impl #generics Eq for #ident #generics {}
    }.into()
}

fn is_named_struct(fields: &darling::ast::Fields<syn::Field>) -> bool {
    return fields
        .iter()
        .all(|field| field.ident.is_some());
}

fn generate_fields_for_named_struct(fields: darling::ast::Fields<syn::Field>) -> Vec<syn::Member> {
    fields
        .iter()
        .filter_map(|field| field.ident.clone().map(syn::Member::Named)) // Convert Ident to Member::Named
        .collect()
}

fn generate_fields_for_unnamed_struct(fields: darling::ast::Fields<syn::Field>) -> Vec<syn::Member> {
    (0..fields.len())
        .map(|i| syn::Member::Unnamed(syn::Index::from(i)))
        .collect()
}
