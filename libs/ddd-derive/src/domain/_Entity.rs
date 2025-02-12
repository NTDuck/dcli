use crate::utils::tokenize;
use crate::utils::Field;
use crate::utils::TokenStream;
use crate::utils::StructAbstractSyntaxTree;

pub fn deriveEntity(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return error.write_errors().into(),
    };

    return match ast.data.is_struct() {
        true => generateTokensFromStructAst(StructAbstractSyntaxTree::from(ast)),
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

fn generateTokensFromStructAst(ast: StructAbstractSyntaxTree<EntityField>) -> TokenStream {
    let field_idents = ast.get_field_idents();

    let StructAbstractSyntaxTree {
        ident,
        generics,
        fields,
        ..
    } = ast;

    let id_field = fields
        .iter()
        .find(|field| isIdField(&field))
        .unwrap();

    let id_field_ident = id_field.ident.as_ref().unwrap();
    let id_field_ty = &id_field.ty;

    return tokenize! {
        impl #generics ddd::domain::Entity for #ident #generics {
            type Id = #id_field_ty;

            fn getId(&self) -> &Self::Id {
                return &self.#id_field_ident;
            }
        }

        impl #generics ddd::domain::ValueObject for #ident #generics {}

        impl #generics std::fmt::Debug for #ident #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #(.field(stringify!(#field_idents), &self.#field_idents))*
                    .finish()
            }
        }

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                return Self {
                    #(#field_idents: self.#field_idents.clone()),*
                };
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                use ddd::domain::Entity;

                return self.getId() == other.getId();
            }
        }

        impl #generics Eq for #ident #generics {}
    };
}

fn isIdField(field: &EntityField) -> bool {
    return fieldHasIdAttribute(field);
}

fn fieldHasIdAttribute(field: &EntityField) -> bool {
    return field.attributes
        .as_ref()
        .map_or(false,
            |attributes| attributes.Identifier.is_some());
}