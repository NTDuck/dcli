use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

pub fn derive_value_object(input_token_stream: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input_token_stream as syn::DeriveInput);

    let DerivedPayload {
        ident,
        generics,
        data,
        ..
    } = match DerivedPayload::from_derive_input(&derive_input) {
        Ok(payload) => payload,
        Err(error) => return TokenStream::from(error.write_errors()),
    };

    match data {
        darling::ast::Data::Struct(fields) => {
            let parsed_payload = ParsedPayload {
                ident,
                generics,
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
struct DerivedPayload {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), syn::Field>,
}

struct ParsedPayload {
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<syn::Field>,
}

fn is_named_struct(fields: &darling::ast::Fields<syn::Field>) -> bool {
    return fields
        .iter()
        .all(|field| field.ident.is_some());
}

fn generate_token_stream_for_named_struct(parsed_payload: ParsedPayload) -> TokenStream {
    let ParsedPayload {
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

fn generate_token_stream_for_unnamed_struct(parsed_payload: ParsedPayload) -> TokenStream {
    let ParsedPayload {
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
