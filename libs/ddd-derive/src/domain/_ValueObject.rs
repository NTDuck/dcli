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
            let parsed_payload = StructPayload {
                ident: payload.ident,
                generics: payload.generics,
                fields,
            };

            if is_named_struct(&parsed_payload.fields) {
                generate_token_stream_for_named_struct(parsed_payload)
            } else {
                generate_token_stream_for_unnamed_struct(parsed_payload)
            }
        }
        _ => unreachable!(),
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

fn generate_token_stream_for_named_struct(parsed_payload: StructPayload) -> TokenStream {
    let StructPayload {
        ident,
        generics,
        fields,
    } = parsed_payload;

    let field_names: Vec<_> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();

    quote! {
        impl #generics ddd::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                Self {
                    #(#field_names: self.#field_names.clone(),)*
                }
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                true #( && self.#field_names == other.#field_names)*
            }
        }

        impl #generics Eq for #ident #generics {}
    }
    .into()
}

fn generate_token_stream_for_unnamed_struct(parsed_payload: StructPayload) -> TokenStream {
    let StructPayload {
        ident,
        generics,
        fields,
    } = parsed_payload;

    let field_indices: Vec<_> = (0..fields.len())
        .map(syn::Index::from)
        .collect();

    quote! {
        impl #generics ddd::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                Self (
                    #(self.#field_indices.clone(),)*
                )
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                true #( && self.#field_indices == other.#field_indices)*
            }
        }

        impl #generics Eq for #ident #generics {}
    }
    .into()
}
