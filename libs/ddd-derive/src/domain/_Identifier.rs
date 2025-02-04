use quote::quote;

use crate::utils::*;

pub fn derive_identifier(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return TokenStream::from(error.into_compile_error()),
    };

    let payload = match DefaultPayload::try_from(ast) {
        Ok(payload) => payload,
        Err(error) => return TokenStream::from(error.write_errors()),
    };

    match payload.data {
        darling::ast::Data::Struct(_) => {
            let payload = DefaultStructPayload::from(payload);
            return generate_tokens_from_struct_payload(payload);
        },
        darling::ast::Data::Enum(_) => {
            todo!()
        },
    }
}

fn generate_tokens_from_struct_payload(payload: DefaultStructPayload) -> TokenStream {
    let DefaultStructPayload {
        ident,
        generics,
        fields,
    } = payload;

    return quote! {
        impl #generics ddd::domain::Identifier for #ident #generics {}
        impl #generics ddd::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                return Self {
                    #(#fields: self.#fields.clone(), )*
                };
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                return true #( && self.#fields == other.#fields)*;
            }
        }

        impl #generics Eq for #ident #generics {}
    }.into();
}
