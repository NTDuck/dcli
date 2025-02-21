use quote::format_ident;
use quote::quote;

use crate::utils::ast::getBoundedWhereClauseFromBoundsAndWhereClause;
use crate::utils::ast::getGenericIdentsFromDeriveInput;
use crate::utils::derives::deriveDebugForNamedStruct;
use crate::utils::derives::deriveDebugForStructVariant;
use crate::utils::derives::deriveDebugForTupleStruct;
use crate::utils::derives::deriveDebugForTupleVariant;
use crate::utils::derives::deriveDebugForUnitStruct;
use crate::utils::derives::deriveDebugForUnitVariant;
use crate::utils::derives::getDebugBoundedWhereClauseFromDeriveInput;

pub fn deriveSerdelessDataTransferObject(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        syn::Data::Enum(data) => deriveForEnum(&ast, data),
        _ => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn deriveForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    return match fields {
        syn::Fields::Named(fields) => deriveForNamedStruct(ast, fields),
        syn::Fields::Unnamed(fields) => deriveForTupleStruct(ast, fields),
        syn::Fields::Unit => deriveForUnitStruct(ast),
    };
}

fn deriveForNamedStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();

    let dataTransferObjectBoundedStructWhereClause = getDataTransferObjectBoundedWhereClause(ast);
    let cloneBoundedStructWhereClause = getCloneBoundedWhereClauseFromDeriveInput(ast);

    let debugImpl = deriveDebugForNamedStruct(ast, fields);

    let fieldIdents = fields.named
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    return quote! {
        impl #structImplGenerics axiom::interfaces::DataTransferObject for #structIdent #structTypeGenerics #dataTransferObjectBoundedStructWhereClause {}

        #debugImpl

        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #cloneBoundedStructWhereClause {
            fn clone(&self) -> Self {
                return Self {
                    #( #fieldIdents: self.#fieldIdents.clone(), )*
                };
            }
        }
    };
}

fn deriveForTupleStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();

    let dataTransferObjectBoundedStructWhereClause = getDataTransferObjectBoundedWhereClause(ast);
    let cloneBoundedStructWhereClause = getCloneBoundedWhereClauseFromDeriveInput(ast);

    let debugImpl = deriveDebugForTupleStruct(ast, fields);

    let fieldIndices = (0..fields.unnamed.len())
        .map(syn::Index::from)
        .collect::<Vec<_>>();

    return quote! {
        impl #structImplGenerics axiom::interfaces::DataTransferObject for #structIdent #structTypeGenerics #dataTransferObjectBoundedStructWhereClause {}

        #debugImpl

        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #cloneBoundedStructWhereClause {
            fn clone(&self) -> Self {
                return Self( #( self.#fieldIndices.clone(), )* );
            }
        }
    };
}

fn deriveForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, _) = ast.generics.split_for_impl();

    let dataTransferObjectBoundedStructWhereClause = getDataTransferObjectBoundedWhereClause(ast);
    let cloneBoundedStructWhereClause = getCloneBoundedWhereClauseFromDeriveInput(ast);

    let debugImpl = deriveDebugForUnitStruct(ast);

    return quote! {
        impl #structImplGenerics axiom::interfaces::DataTransferObject for #structIdent #structTypeGenerics #dataTransferObjectBoundedStructWhereClause {}

        #debugImpl

        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #cloneBoundedStructWhereClause {
            fn clone(&self) -> Self {
                return Self;
            }
        }
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, _) = ast.generics.split_for_impl();

    let dataTransferObjectBoundedEnumWhereClause = getDataTransferObjectBoundedWhereClause(ast);
    let debugBoundedEnumWhereClause = getDebugBoundedWhereClauseFromDeriveInput(ast);
    let cloneBoundedEnumWhereClause = getCloneBoundedWhereClauseFromDeriveInput(ast);
    
    let variantDebugImpls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => deriveDebugForStructVariant(ast, variant, fields),
            syn::Fields::Unnamed(fields) => deriveDebugForTupleVariant(ast, variant, fields),
            syn::Fields::Unit => deriveDebugForUnitVariant(variant),
        })
        .collect::<Vec<_>>();

    let variantCloneImpls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => deriveCloneForNamedVariant(variant, fields),
            syn::Fields::Unnamed(fields) => deriveCloneForUnnamedVariant(variant, fields),
            syn::Fields::Unit => deriveCloneForUnitVariant(variant),
        })
        .collect::<Vec<_>>();

    return quote! {
        impl #enumImplGenerics axiom::interfaces::DataTransferObject for #enumIdent #enumTypeGenerics #dataTransferObjectBoundedEnumWhereClause {}

        impl #enumImplGenerics std::fmt::Debug for #enumIdent #enumTypeGenerics #debugBoundedEnumWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    #( #variantDebugImpls, )*
                };
            }
        }

        impl #enumImplGenerics Clone for #enumIdent #enumTypeGenerics #cloneBoundedEnumWhereClause {
            fn clone(&self) -> Self {
                return match self {
                    #( #variantCloneImpls, )*
                };
            }
        }
    };
}

fn deriveCloneForNamedVariant(variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let fieldIdents = fields.named
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    return quote! {
        Self::#variantIdent { #( #fieldIdents, )* } => 
            Self::#variantIdent { #( #fieldIdents: #fieldIdents.clone(), )* }
    };
}

fn deriveCloneForUnnamedVariant(variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let fieldIdents = (0..fields.unnamed.len())
        .map(|index| format_ident!("arg{index}"))
        .collect::<Vec<_>>();

    return quote! {
        Self::#variantIdent(#( #fieldIdents, )*) => 
            Self::#variantIdent(#( #fieldIdents.clone(), )*)
    };
}

fn deriveCloneForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;

    return quote! {
        Self::#variantIdent => Self::#variantIdent
    };
}

fn getDataTransferObjectBoundedWhereClause(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let genericIdents = getGenericIdentsFromDeriveInput(ast);
    let bounds = genericIdents
        .iter()
        .map(|ident| quote! {
            #ident: axiom::interfaces::DataTransferObject
        });

    let (_, _, whereClause) = ast.generics.split_for_impl();
    
    return getBoundedWhereClauseFromBoundsAndWhereClause(bounds, whereClause);
}

fn getCloneBoundedWhereClauseFromDeriveInput(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let genericIdents = getGenericIdentsFromDeriveInput(ast);
    let bounds = genericIdents
        .iter()
        .map(|ident| quote! {
            #ident: Clone
        });

    let (_, _, whereClause) = ast.generics.split_for_impl();
    
    return getBoundedWhereClauseFromBoundsAndWhereClause(bounds, whereClause);
}
