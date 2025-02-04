use proc_macro::TokenStream;
use quote::quote;

use crate::utils::*;

pub fn derive_entity(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return TokenStream::from(error.into_compile_error()),
    };

    let payload = match Payload::try_from(ast) {
        Ok(payload) => payload,
        Err(error) => return TokenStream::from(error.write_errors()),
    };

    let payload = StructPayload::from(payload);

    return generate_tokens_from_payload(payload);
}

#[derive(darling::FromDeriveInput)]
#[darling(attributes(ddd), supports(struct_named))]
struct Payload {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<darling::util::Ignored, Field>,
}

impl TryFrom<AbstractSyntaxTree> for Payload {
    type Error = darling::Error;

    fn try_from(ast: AbstractSyntaxTree) -> Result<Self, Self::Error> {
        use darling::FromDeriveInput;
        return Self::from_derive_input(&ast);
    }
}

#[derive(darling::FromField)]
#[darling(attributes(ddd))]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    
    field: Option<FieldAttributes>,
}

#[derive(darling::FromMeta)]
struct FieldAttributes {
    Identifier: Option<IdMarker>,
}

#[derive(darling::FromMeta)]
struct IdMarker;

struct StructPayload {
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<Field>,
}

impl From<Payload> for StructPayload {
    fn from(payload: Payload) -> Self {
        return Self {
            ident: payload.ident,
            generics: payload.generics,
            fields: payload.data.take_struct().unwrap(),
        };
    }
}

fn generate_tokens_from_payload(payload: StructPayload) -> TokenStream {
    let StructPayload {
        ident,
        generics,
        fields,
    } = payload;

    let id_field = fields
        .iter()
        .find(|field| is_id_field(&field))
        .unwrap();

    let id_ident = id_field.ident.as_ref().unwrap();
    let id_type = &id_field.ty;

    let fields = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap());

    quote! {
        impl #generics ddd::domain::Entity for #ident #generics {
            type Id = #id_type;

            fn get_id(&self) -> &Self::Id {
                return &self.#id_ident;
            }
        }

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
                use ddd::domain::Entity;

                return self.get_id() == other.get_id();
            }
        }

        impl #generics Eq for #ident #generics {}
    }.into()
}

fn is_id_field(field: &Field) -> bool {
    return field_has_id_attribute(field);
}

fn field_has_id_attribute(field: &Field) -> bool {
    return field.field
        .as_ref()
        .map_or(false, |attributes| attributes.Identifier.is_some());
}