use crate::utils::tokenize;
use crate::utils::AbstractSyntaxTree;
use crate::utils::Field;
use crate::utils::TokenStream;
use crate::utils::StructAbstractSyntaxTree;

pub fn deriveValueObject(tokens: TokenStream) -> TokenStream {
    let ast: AbstractSyntaxTree::<darling::util::Ignored, Field> =
        match AbstractSyntaxTree::try_from(tokens) {
            Ok(ast) => ast,
            Err(error) => return error.write_errors().into(),
        };

    return match ast.data.is_struct() {
        true => generate_tokens_from_struct_ast(StructAbstractSyntaxTree::from(ast)),
        false => todo!(),
    };
}

fn generate_tokens_from_struct_ast(ast: StructAbstractSyntaxTree<Field>) -> TokenStream {
    let field_idents = ast.get_field_idents();

    let StructAbstractSyntaxTree {
        ident,
        generics,
        ..
    } = ast;

    return tokenize! {
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
                return true #( && self.#field_idents == other.#field_idents)*;
            }
        }

        impl #generics Eq for #ident #generics {}

        impl #generics std::hash::Hash for #ident #generics {
            fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
                #(self.#field_idents.hash(state); )*
            }
        }
    };
}
