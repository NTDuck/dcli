use quote::format_ident;
use quote::quote;

use crate::utils::ast::convertIdentToCamelCase;
use crate::utils::ast::getFieldIdentsFromNamedFields;

pub fn deriveNew(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
        syn::Fields::Unnamed(fields) => deriveForUnnamedStruct(ast, fields),
        syn::Fields::Unit => deriveForUnitStruct(ast),
    };
}

fn deriveForNamedStruct(ast: &syn::DeriveInput, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();
    let methodIdent = getMethodIdentForStruct();

    let fieldIdents = getFieldIdentsFromNamedFields(fields);
    let fieldTypes = getFieldTypesFromNamedFields(fields);

    return quote! {
        impl #structImplGenerics #structIdent #structTypeGenerics #structWhereClause {
            pub fn #methodIdent(#( #fieldIdents: #fieldTypes, )*) -> Self {
                return Self { #( #fieldIdents, )* };
            }
        }
    };
}

fn deriveForUnnamedStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();
    let methodIdent = getMethodIdentForStruct();

    let fieldIdents = getFieldIdentsWithArgPrefixedFromUnnamedFields(fields);
    let fieldTypes = getFieldTypesFromUnnamedFields(fields);

    return quote! {
        impl #structImplGenerics #structIdent #structTypeGenerics #structWhereClause {
            pub fn #methodIdent(#( #fieldIdents: #fieldTypes, )*) -> Self {
                return Self( #( #fieldIdents, )* );
            }
        }
    };
}

fn deriveForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();
    let methodIdent = getMethodIdentForStruct();

    return quote! {
        impl #structImplGenerics #structIdent #structTypeGenerics #structWhereClause {
            pub const fn #methodIdent() -> Self {
                return Self;
            }
        }
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let variantNewMethods = variants
        .iter()
        .map(|variant| {
            return match &variant.fields {
                syn::Fields::Named(fields) => deriveForNamedVariant(variant, fields),
                syn::Fields::Unnamed(fields) => deriveForUnnamedVariant(variant, fields),
                syn::Fields::Unit => deriveForUnitVariant(variant),
            };
        })
        .collect::<Vec<_>>();

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, enumWhereClause) = ast.generics.split_for_impl();

    return quote! {
        impl #enumImplGenerics #enumIdent #enumTypeGenerics #enumWhereClause {
            #( #variantNewMethods )*
        }
    }
}

fn deriveForNamedVariant(variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let methodIdent = getMethodIdentForEnumFromVariant(variant);

    let fieldIdents = getFieldIdentsFromNamedFields(fields);
    let fieldTypes = getFieldTypesFromNamedFields(fields);

    return quote! {
        pub fn #methodIdent(#( #fieldIdents: #fieldTypes, )*) -> Self {
            return Self::#variantIdent { #(#fieldIdents, )* };
        }
    };
}

fn deriveForUnnamedVariant(variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let methodIdent = getMethodIdentForEnumFromVariant(variant);

    let fieldIdents = getFieldIdentsWithArgPrefixedFromUnnamedFields(fields);
    let fieldTypes = getFieldTypesFromUnnamedFields(fields);

    return quote! {
        pub fn #methodIdent(#( #fieldIdents: #fieldTypes, )*) -> Self {
            return Self::#variantIdent( #( #fieldIdents, )* );
        }
    };
}

fn deriveForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let methodIdent = getMethodIdentForEnumFromVariant(variant);

    return quote! {
        pub fn #methodIdent() -> Self {
            return Self::#variantIdent;
        }
    };
}

fn getMethodIdentForStruct() -> syn::Ident {
    return format_ident!("{BaseMethodIdent}");
}

fn getFieldTypesFromNamedFields(fields: &syn::FieldsNamed) -> Vec<&syn::Type> {
    return fields.named
        .iter()
        .map(|field| &field.ty)
        .collect();
}

fn getFieldIdentsWithArgPrefixedFromUnnamedFields(fields: &syn::FieldsUnnamed) -> Vec<syn::Ident> {
    return (0..fields.unnamed.len())
        .map(|index| format_ident!("arg{index}"))
        .collect::<Vec<_>>();
}

fn getFieldTypesFromUnnamedFields(fields: &syn::FieldsUnnamed) -> Vec<&syn::Type> {
    return fields.unnamed
        .iter()
        .map(|field| &field.ty)
        .collect();
}

fn getMethodIdentForEnumFromVariant(variant: &syn::Variant) -> syn::Ident {
    let variantIdent = &variant.ident;

    let unformattedMethodIdent = format_ident!("{BaseMethodIdent}{variantIdent}");
    let formattedMethodIdent = convertIdentToCamelCase(&unformattedMethodIdent);

    return formattedMethodIdent;
}

const BaseMethodIdent: &str = "new";
