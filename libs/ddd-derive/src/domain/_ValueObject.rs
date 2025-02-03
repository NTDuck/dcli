use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput)]
#[darling(supports(struct_named))]
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

    let fields = data.take_struct().unwrap();
    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    quote! {
        impl #generics ddd_rs::domain::ValueObject for #ident #generics {}

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
    }.into()
}
