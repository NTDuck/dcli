use proc_macro::TokenStream;
use quote::quote;

use crate::utils::*;

pub fn derive_entity(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return error.write_errors().into(),
    };

    return match ast.data.is_struct() {
        true => generate_tokens_from_struct_ast(StructAbstractSyntaxTree::from(ast)),
        false => todo!(),
    }
}

type AbstractSyntaxTree = crate::utils::AbstractSyntaxTree<darling::util::Ignored, EntityField>;
type EntityField = Field<FieldAttributes>;

#[derive(darling::FromMeta)]
struct FieldAttributes {
    Identifier: Option<IdMarker>,
}

#[derive(darling::FromMeta)]
struct IdMarker;

fn generate_tokens_from_struct_ast(ast: StructAbstractSyntaxTree<EntityField>) -> TokenStream {
    let StructAbstractSyntaxTree {
        ident,
        generics,
        fields,
        ..
    } = ast;

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

fn is_id_field(field: &EntityField) -> bool {
    return field_has_id_attribute(field);
}

fn field_has_id_attribute(field: &EntityField) -> bool {
    return field.attributes
        .as_ref()
        .map_or(false, |attributes| attributes.Identifier.is_some());
}