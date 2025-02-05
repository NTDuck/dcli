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

type AbstractSyntaxTree = crate::utils::AbstractSyntaxTree<darling::util::Ignored, syn::Field>;

fn generate_tokens_from_struct_ast(ast: StructAbstractSyntaxTree<syn::Field>) -> TokenStream {
    let StructAbstractSyntaxTree {
        ident,
        generics,
        fields,
        ..
    } = ast;

    let fields: Vec<_> = fields
        .iter()
        .filter_map(|field| field.ident.clone())
        .map(|ident| syn::Member::Named(ident))
        .collect();

    return quote! {
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
                return true #( && self.#fields == other.#fields)*;
            }
        }

        impl #generics Eq for #ident #generics {}
    }.into();
}
