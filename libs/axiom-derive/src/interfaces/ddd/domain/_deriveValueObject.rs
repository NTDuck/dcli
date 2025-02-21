use quote::quote;

use crate::utils::ast::*;
use crate::utils::derives::*;

pub fn deriveValueObject(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let structWhereClauseWithValueObjectBounds = generateWhereClauseWithValueObjectBoundsFromDeriveInput(ast);
    
    let structDebugImpl = deriveDebugForStruct(ast, data);
    let structCloneImpl = deriveCloneForStruct(ast, data);
    let structPartialEqImpl = derivePartialEqForStruct(ast, data);
    let structEqImpl = deriveEq(ast);

    return quote! {
        impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClauseWithValueObjectBounds {}

        #structDebugImpl
        #structCloneImpl
        #structPartialEqImpl
        #structEqImpl
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();

    let enumWhereClauseWithValueObjectBounds = generateWhereClauseWithValueObjectBoundsFromDeriveInput(ast);

    let enumDebugImpl = deriveDebugForEnum(ast, data);
    let enumCloneImpl = deriveCloneForEnum(ast, data);
    let enumPartialEqimpl = derivePartialEqForEnum(ast, data);
    let enumEqImpl = deriveEq(ast);

    return quote! {
        impl #enumImplGenerics axiom::interfaces::ddd::domain::ValueObject for #enumIdent #enumTypeGenerics #enumWhereClauseWithValueObjectBounds {}

        #enumDebugImpl
        #enumCloneImpl
        #enumPartialEqimpl
        #enumEqImpl
    };
}

fn generateWhereClauseWithValueObjectBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    return generateWhereClauseWithTraitBoundsFromDeriveInput(
        |T| quote! {
            #T: axiom::interfaces::ddd::domain::ValueObject
        },
        ast,
    );
}
