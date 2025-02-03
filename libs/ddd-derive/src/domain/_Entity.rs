use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput)]
#[darling(attributes(entity), supports(struct_named))]
struct EntityPayload {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<darling::util::Ignored, EntityField>,
}

#[derive(darling::FromMeta)]
struct IdMarker;

#[derive(darling::FromField)]
#[darling(attributes(entity))]
struct EntityField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    id: Option<IdMarker>,
}

pub fn derive_entity(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let EntityPayload {
        ident,
        generics,
        data,
        ..
    } = match EntityPayload::from_derive_input(&derive_input) {
        Ok(entity) => entity,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let fields = data.take_struct().unwrap();

    derive_entity_impl(ident, generics, fields)
}

fn derive_entity_impl(
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<EntityField>,
) -> TokenStream {
    let id_field = fields
        .into_iter()
        .find(|f| f.id.is_some())
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