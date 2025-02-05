use crate::utils::implementation;
use crate::utils::Field;
use crate::utils::IntermediateTokenStream;
use crate::utils::StructAbstractSyntaxTree;

pub fn generate_PartialEq_impl_for_struct_ast<Attributes>(ast: &StructAbstractSyntaxTree<Field<Attributes>>) -> IntermediateTokenStream
where
    Attributes: darling::FromMeta,
{
    let StructAbstractSyntaxTree {
        ident,
        generics,
        ..
    } = ast;

    let field_idents = ast.get_field_idents();

    return implementation! {
        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                return true #( && self.#field_idents == other.#field_idents)*;
            }
        }
    };
}
