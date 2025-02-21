use quote::quote;

use crate::utils::ast::*;
use crate::utils::derives::*;

pub fn deriveIdentifier(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let structWhereClauseWithIdentifierBounds = getWhereClauseWithIdentifierBoundsFromDeriveInput(ast);
    let structWhereClauseWithValueObjectBounds = getWhereClauseWithValueObjectBoundsFromDeriveInput(ast);
    
    let structDebugImpl = deriveDebugForStruct(ast, data);
    let structCloneImpl = deriveCloneForStruct(ast, data);
    let structPartialEqImpl = derivePartialEqForStruct(ast, data);
    let structEqImpl = deriveEq(ast);
    let structHashImpl = deriveHashForStruct(ast, data);

    return quote! {
        impl #structImplGenerics axiom::interfaces::ddd::domain::Identifier for #structIdent #structTypeGenerics #structWhereClauseWithIdentifierBounds {}

        impl #structImplGenerics axiom::interfaces::ddd::domain::ValueObject for #structIdent #structTypeGenerics #structWhereClauseWithValueObjectBounds {}

        #structDebugImpl
        #structCloneImpl
        #structPartialEqImpl
        #structEqImpl
        #structHashImpl
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();

    let enumWhereClauseWithIdentifierBounds = getWhereClauseWithIdentifierBoundsFromDeriveInput(ast);
    let enumWhereClauseWithValueObjectBounds = getWhereClauseWithValueObjectBoundsFromDeriveInput(ast);

    let enumDebugImpl = deriveDebugForEnum(ast, data);
    let enumCloneImpl = deriveCloneForEnum(ast, data);
    let enumPartialEqimpl = derivePartialEqForEnum(ast, data);
    let enumEqImpl = deriveEq(ast);
    let enumHashImpl = deriveHashForEnum(ast, data);

    return quote! {
        impl #enumImplGenerics axiom::interfaces::ddd::domain::Identifier for #enumIdent #enumTypeGenerics #enumWhereClauseWithIdentifierBounds {}

        impl #enumImplGenerics axiom::interfaces::ddd::domain::ValueObject for #enumIdent #enumTypeGenerics #enumWhereClauseWithValueObjectBounds {}

        #enumDebugImpl
        #enumCloneImpl
        #enumPartialEqimpl
        #enumEqImpl
        #enumHashImpl
    };
}

fn getWhereClauseWithIdentifierBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let (_, _, whereClause) = ast.generics.split_for_impl();

    let genericIdents = getGenericIdentsFromDeriveInput(ast);
    let identifierBounds = genericIdents
        .iter()
        .map(|ident| quote! {
            #ident: axiom::interfaces::ddd::domain::Identifier
        });
    
    return getWhereClauseWithTraitBoundsFromWhereClauseAndTraitBounds(whereClause, identifierBounds);
}

fn getWhereClauseWithValueObjectBoundsFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let (_, _, whereClause) = ast.generics.split_for_impl();

    let genericIdents = getGenericIdentsFromDeriveInput(ast);
    let valueObjectBounds = genericIdents
        .iter()
        .map(|ident| quote! {
            #ident: axiom::interfaces::ddd::domain::ValueObject
        });
    
    return getWhereClauseWithTraitBoundsFromWhereClauseAndTraitBounds(whereClause, valueObjectBounds);
}
