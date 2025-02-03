use proc_macro::TokenStream;
use quote::quote;

use crate::utils::*;

pub fn derive_entity(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return TokenStream::from(error.into_compile_error()),
    };

    let payload = match EntityPayload::try_from(ast) {
        Ok(payload) => payload,
        Err(error) => return TokenStream::from(error.write_errors()),
    };

    let payload = BetterPayload {
        ident: payload.ident,
        generics: payload.generics,
        fields: payload.data.take_struct().unwrap(),
    };

    return generate_tokens_from_payload(payload);
}

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

#[derive(darling::FromField)]
#[darling(attributes(entity))]
struct EntityField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    
    id: Option<IdMarker>,
}

#[derive(darling::FromMeta)]
struct IdMarker;

struct BetterPayload {
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<EntityField>,
}

fn generate_tokens_from_payload(payload: BetterPayload) -> TokenStream {
    let BetterPayload {
        ident,
        generics,
        fields,
    } = payload;

    let id_field = fields
        .iter()
        .find(|f| f.id.is_some() || f.ident.as_ref().map(|ident| ident == "id").unwrap_or(false))
        .expect("Missing `id` field");

    let id_ident = id_field.ident.as_ref().unwrap();
    let id_ty = &id_field.ty;

    let clone_fields = fields
        .iter()
        .map(|field| {
            let field_ident = field.ident.as_ref().unwrap();
            quote! { #field_ident: self.#field_ident.clone(), }
        });

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

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                Self {
                    #(#clone_fields)*
                }
            }
        }
    }
    .into()
}
