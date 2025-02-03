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
            let payload = DefaultEnumPayload::from(payload);
            return generate_tokens_from_enum_payload(payload);
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

fn generate_tokens_from_enum_payload(payload: DefaultEnumPayload) -> TokenStream {
    let DefaultEnumPayload {
        ident,
        generics,
        variants,
    } = payload;

    let variant_impls = variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;

            return quote! {
                impl #generics Clone for #ident #generics {
                    fn clone(&self) -> Self {
                        match self {
                            #ident::#variant_ident(ref val) => #ident::#variant_ident(val.clone()),
                        }
                    }
                }

                impl #generics PartialEq for #ident #generics {
                    fn eq(&self, other: &Self) -> bool {
                        match (self, other) {
                            (#ident::#variant_ident(ref val1), #ident::#variant_ident(ref val2)) => val1 == val2,
                            _ => false,
                        }
                    }
                }

                impl #generics Eq for #ident #generics {}
            };
        });

    return quote! {
        impl #generics ddd::domain::Identifier for #ident #generics {}
        impl #generics ddd::domain::ValueObject for #ident #generics {}

        #(#variant_impls)*
    }.into();
}
