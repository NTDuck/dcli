use quote::quote;

use crate::utils::ast::*;
use crate::utils::derives::*;

pub fn deriveDataTransferObject(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        syn::Data::Enum(data) => deriveForEnum(&ast, data),
        _ => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn deriveForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();

    let structWhereClauseWithDataTransferObjectBounds = generateWhereClauseWithDataTransferObjectBoundsFromDeriveInput(ast);
    
    let structDebugImpl = deriveDebugForStruct(ast, data);
    let structCloneImpl = deriveCloneForStruct(ast, data);
    let structSerializeImpl = deriveSerializeForStruct(ast, data);
    let structDeserializeImpl = deriveDeserializeForStruct(ast, data);

    return quote! {
        impl #structImplGenerics axiom::interfaces::DataTransferObject for #structIdent #structTypeGenerics #structWhereClauseWithDataTransferObjectBounds {}

        #structDebugImpl
        #structCloneImpl
        #structSerializeImpl
        #structDeserializeImpl
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();

    let enumWhereClauseWithDataTransferObjectBounds = generateWhereClauseWithDataTransferObjectBoundsFromDeriveInput(ast);

    let enumDebugImpl = deriveDebugForEnum(ast, data);
    let enumCloneImpl = deriveCloneForEnum(ast, data);
    let enumSerializeImpl = deriveSerializeForEnum(ast, data);
    let enumDeserializeImpl = deriveDeserializeForEnum(ast, data);

    return quote! {
        impl #enumImplGenerics axiom::interfaces::DataTransferObject for #enumIdent #enumTypeGenerics #enumWhereClauseWithDataTransferObjectBounds {}

        #enumDebugImpl
        #enumCloneImpl
        #enumSerializeImpl
        #enumDeserializeImpl
    };
}

fn generateWhereClauseWithDataTransferObjectBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: axiom::interfaces::DataTransferObject
        },
        ast,
    );
}
