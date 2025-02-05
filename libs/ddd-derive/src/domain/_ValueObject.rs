use quote::quote;

use crate::utils::*;

pub fn derive_value_object(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return error.write_errors().into(),
    };

    match ast.data.is_struct() {
        true => generate_tokens_from_struct_ast(StructAbstractSyntaxTree::from(ast)),
        false => todo!(),
    }
}

type AbstractSyntaxTree = crate::utils::AbstractSyntaxTree<darling::util::Ignored, Field>;

fn generate_tokens_from_struct_ast(ast: StructAbstractSyntaxTree<Field>) -> TokenStream {
    let field_idents = ast.get_field_idents();

    let StructAbstractSyntaxTree {
        ident,
        generics,
        ..
    } = ast;

    return quote! {
        impl #generics ddd::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                return Self {
                    #(#field_idents: self.#field_idents.clone(), )*
                };
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                return true #( && self.#field_idents == other.#field_idents)*;
            }
        }

        impl #generics Eq for #ident #generics {}
    }.into();
}
