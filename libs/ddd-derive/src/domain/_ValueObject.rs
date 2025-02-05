use crate::utils::generate_Clone_impl_for_struct_ast;
use crate::utils::generate_Copy_impl_for_struct_ast;
use crate::utils::generate_Eq_impl_for_struct_ast;
use crate::utils::generate_PartialEq_impl_for_struct_ast;
use crate::utils::tokenize;
use crate::utils::Field;
use crate::utils::TokenStream;
use crate::utils::StructAbstractSyntaxTree;

pub fn derive_value_object(tokens: TokenStream) -> TokenStream {
    let ast = match AbstractSyntaxTree::try_from(tokens) {
        Ok(ast) => ast,
        Err(error) => return error.write_errors().into(),
    };

    return match ast.data.is_struct() {
        true => generate_tokens_from_struct_ast(StructAbstractSyntaxTree::from(ast)),
        false => todo!(),
    };
}

type AbstractSyntaxTree = crate::utils::AbstractSyntaxTree<darling::util::Ignored, Field>;

fn generate_tokens_from_struct_ast(ast: StructAbstractSyntaxTree<Field>) -> TokenStream {
    let Clone_impl = generate_Clone_impl_for_struct_ast(&ast);
    let PartialEq_impl = generate_PartialEq_impl_for_struct_ast(&ast);
    let Eq_impl = generate_Eq_impl_for_struct_ast(&ast);
    
    let StructAbstractSyntaxTree {
        ident,
        generics,
        ..
    } = ast;

    return tokenize! {
        impl #generics ddd::domain::ValueObject for #ident #generics {}

        #Clone_impl
        #PartialEq_impl
        #Eq_impl
    };
}
