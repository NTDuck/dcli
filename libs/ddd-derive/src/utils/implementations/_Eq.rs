use crate::utils::implementation;
use crate::utils::Field;
use crate::utils::IntermediateTokenStream;
use crate::utils::StructAbstractSyntaxTree;

pub fn generate_Eq_impl_for_struct_ast<Attributes>(ast: &StructAbstractSyntaxTree<Field<Attributes>>) -> IntermediateTokenStream
where
    Attributes: darling::FromMeta,
{
    let StructAbstractSyntaxTree {
        ident,
        generics,
        ..
    } = ast;

    return implementation! {
        impl #generics Eq for #ident #generics {}
    };
}
