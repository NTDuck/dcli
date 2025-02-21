use quote::quote;

use crate::utils::ast::*;
use crate::utils::derives::*;

pub fn deriveDataTransferObjectWithoutSerde(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let structWhereClauseWithDataTransferObjectBounds = getDataTransferObjectBoundedWhereClause(ast);
    
    let structCloneImpl = deriveCloneForStruct(ast, data);
    let structDebugImpl = deriveDebugForStruct(ast, data);

    return quote! {
        impl #structImplGenerics axiom::interfaces::DataTransferObject for #structIdent #structTypeGenerics #structWhereClauseWithDataTransferObjectBounds {}

        #structDebugImpl
        #structCloneImpl
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();

    let structWhereClauseWithDataTransferObjectBounds = getDataTransferObjectBoundedWhereClause(ast);

    let enumDebugImpl = deriveDebugForEnum(ast, data);
    let enumCloneImpl = deriveCloneForEnum(ast, data);

    return quote! {
        impl #enumImplGenerics axiom::interfaces::DataTransferObject for #enumIdent #enumTypeGenerics #structWhereClauseWithDataTransferObjectBounds {}

        #enumDebugImpl
        #enumCloneImpl
    };
}

fn getDataTransferObjectBoundedWhereClause(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let (_, _, whereClause) = ast.generics.split_for_impl();

    let genericIdents = getGenericIdentsFromDeriveInput(ast);
    let dataTransferObjectBounds = genericIdents
        .iter()
        .map(|ident| quote! {
            #ident: axiom::interfaces::DataTransferObject
        });
    
    return getWhereClauseWithTraitBoundsFromWhereClauseAndTraitBounds(whereClause, dataTransferObjectBounds);
}
