use quote::quote;

use crate::utils::*;

pub fn derive_identifier(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return TokenStream::from(error.into_compile_error()),
    };

    let payload = match Payload::try_from(ast) {
        Ok(payload) => payload,
        Err(error) => return TokenStream::from(error.write_errors()),
    };

    match payload.data.clone() {
        darling::ast::Data::Struct(fields) => {
            let payload = StructPayload::from((payload, fields));
            return generate_tokens_from_struct_payload(payload);
        },
        darling::ast::Data::Enum(_variants) => {
            todo!()
        },
    }
}

fn generate_tokens_from_struct_payload(payload: StructPayload) -> TokenStream {
    let StructPayload {
        ident,
        generics,
        fields,
    } = payload;

    let field = fields;

    return quote! {
        impl #generics ddd::domain::Identifier for #ident #generics {}
        impl #generics ddd::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                return Self {
                    #(#field: self.#field.clone(), )*
                }
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                return true #( && self.#field == other.#field)*;
            }
        }

        impl #generics Eq for #ident #generics {}
    }.into()
}
