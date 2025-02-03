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
        darling::ast::Data::Enum(_variants) => {
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
        let fields = match self.is_named_struct() {
            true => self.generate_fields_for_named_struct(),
            false => self.generate_fields_for_unnamed_struct(),
        };

        return self.generate_tokens_from_fields(fields);
    }
}

impl StructPayload {
    fn is_named_struct(&self) -> bool {
        return self.fields
            .iter()
            .all(|field| field.ident.is_some());
    }
    
    fn generate_fields_for_named_struct(&self) -> Vec<syn::Member> {
        return self.fields
            .iter()
            .filter_map(|field| field.ident.clone())
            .map(|ident| syn::Member::Named(ident))
            .collect();
    }

    fn generate_fields_for_unnamed_struct(&self) -> Vec<syn::Member> {
        return (0..self.fields.len())
            .map(|index| syn::Index::from(index))
            .map(|index| syn::Member::Unnamed(index))
            .collect();
    }

    fn generate_tokens_from_fields(&self, field: Vec<syn::Member>) -> TokenStream {
        let ident = &self.ident;
        let generics = &self.generics;

        return quote! {
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
}
