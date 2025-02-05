use crate::utils::tokens;
use crate::utils::Field;
use crate::utils::TokenStream;
use crate::utils::StructAbstractSyntaxTree;

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
    let fields_ident = ast.get_field_idents();

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

    let id_field_ident = id_field.ident.as_ref().unwrap();
    let id_field_ty = &id_field.ty;

    return tokens! {
        impl #generics ddd::domain::Entity for #ident #generics {
            type Id = #id_field_ty;

            fn get_id(&self) -> &Self::Id {
                return &self.#id_field_ident;
            }
        }

        impl #generics ddd::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                return Self {
                    #(#fields_ident: self.#fields_ident.clone()),*
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
    };
}

fn is_id_field(field: &EntityField) -> bool {
    return field_has_id_attribute(field);
}

fn field_has_id_attribute(field: &EntityField) -> bool {
    return field.attributes
        .as_ref()
        .map_or(false,
            |attributes| attributes.Identifier.is_some());
}