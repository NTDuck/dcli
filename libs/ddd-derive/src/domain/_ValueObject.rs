use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput)]
#[darling(supports(struct_any))] // Supports both named and unnamed structs
struct Payload {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), syn::Field>,
}

pub fn derive_value_object(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let Payload {
        ident,
        generics,
        data,
        ..
    } = match Payload::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(error) => return TokenStream::from(error.write_errors()),
    };

    match data {
        darling::ast::Data::Struct(fields) => {
            if fields.iter().all(|f| f.ident.is_some()) {
                generate_named_struct(ident, generics, fields)
            } else {
                generate_unnamed_struct(ident, generics, fields)
            }
        }
        _ => unreachable!("This derive macro only supports structs"),
    }
}

fn generate_named_struct(
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<syn::Field>,
) -> TokenStream {
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

fn generate_unnamed_struct(
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<syn::Field>,
) -> TokenStream {
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
