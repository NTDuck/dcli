use proc_macro::TokenStream;
use quote::quote;

use crate::utils::*;

#[derive(darling::FromDeriveInput)]
#[darling(attributes(entity), supports(struct_named))]
struct EntityPayload {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<darling::util::Ignored, EntityField>,
}

impl TryFrom<AbstractSyntaxTree> for EntityPayload {
    type Error = darling::Error;

    fn try_from(ast: AbstractSyntaxTree) -> Result<Self, Self::Error> {
        use darling::FromDeriveInput;
        return Self::from_derive_input(&ast);
    }
}

#[derive(darling::FromMeta, Clone)]
struct IdMarker;

#[derive(darling::FromField, Clone)]
#[darling(attributes(entity))]
struct EntityField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    id: Option<IdMarker>,
}

pub fn derive_entity(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return TokenStream::from(error.into_compile_error()),
    };

    let payload = match EntityPayload::try_from(ast) {
        Ok(payload) => payload,
        Err(error) => return TokenStream::from(error.write_errors()),
    };

    let fields = payload.data.clone().take_struct().unwrap();

    derive_entity_impl(payload, fields)
}

fn derive_entity_impl(
    payload: EntityPayload,
    fields: darling::ast::Fields<EntityField>,
) -> TokenStream {
    let EntityPayload {
        ident,
        generics,
        ..
    } = payload;

    let id_field = fields
        .into_iter()
        .find(|f| f.id.is_some() || f.ident.as_ref().map(|ident| ident == "id").unwrap_or(false))
        .expect("Missing `id` field");

    let id_ident = id_field.ident.unwrap();
    let id_ty = id_field.ty;

    quote! {
        impl #generics ddd::domain::Entity for #ident #generics {
            type Id = #id_ty;

            fn get_id(&self) -> &Self::Id {
                &self.#id_ident
            }
        }

        impl #generics ddd::domain::ValueObject for #ident #generics {}

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                use ddd::domain::Entity;

                self.get_id() == other.get_id()
            }
        }

        impl #generics Eq for #ident #generics {}
    }
    .into()
}