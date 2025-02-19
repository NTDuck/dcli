use quote::format_ident;
use quote::quote;

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

    let fieldIdents = fields.named
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    return quote! {
        impl #structImplGenerics axiom::interfaces::DataTransferObject for #structIdent #structTypeGenerics #structWhereClause {}

        impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#structIdent))
                    #( .field(stringify!(#fieldIdents), &self.#fieldIdents) )*
                    .finish();
            }
        }

        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
            fn clone(&self) -> Self {
                return Self {
                    #( #fieldIdents: self.#fieldIdents.clone(), )*
                };
            }
        }
    };
}

fn deriveForUnnamedStruct(ast: &syn::DeriveInput, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

    let fieldIndices = (0..fields.unnamed.len())
        .map(syn::Index::from)
        .collect::<Vec<_>>();

    return quote! {
        impl #structImplGenerics axiom::interfaces::DataTransferObject for #structIdent #structTypeGenerics #structWhereClause {}

        impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_tuple(stringify!(#structIdent))
                    #( .field(&self.#fieldIndices) )*
                    .finish();
            }
        }

        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
            fn clone(&self) -> Self {
                return Self( #( self.#fieldIndices.clone(), )* );
            }
        }
    };
}

fn deriveForUnitStruct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

    return quote! {
        impl #structImplGenerics axiom::interfaces::DataTransferObject for #structIdent #structTypeGenerics #structWhereClause {}

        impl #structImplGenerics std::fmt::Debug for #structIdent #structTypeGenerics #structWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return formatter
                    .debug_struct(stringify!(#structIdent))
                    .finish();
            }
        }

        impl #structImplGenerics Clone for #structIdent #structTypeGenerics #structWhereClause {
            fn clone(&self) -> Self {
                return Self;
            }
        }
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let variantDebugImpls = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(fields) => deriveDebugForNamedVariant(ast, variant, fields),
            syn::Fields::Unnamed(fields) => deriveDebugForUnnamedVariant(ast, variant, fields),
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

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, enumWhereClause) = ast.generics.split_for_impl();

    return quote! {
        impl #enumImplGenerics axiom::interfaces::DataTransferObject for #enumIdent #enumTypeGenerics #enumWhereClause {}

        impl #enumImplGenerics std::fmt::Debug for #enumIdent #enumTypeGenerics #enumWhereClause {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    #( #variantDebugImpls, )*
                };
            }
        }

        impl #enumImplGenerics Clone for #enumIdent #enumTypeGenerics #enumWhereClause {
            fn clone(&self) -> Self {
                return match self {
                    #( #variantCloneImpls, )*
                };
            }
        }
    };
}

fn deriveDebugForNamedVariant(ast: &syn::DeriveInput, variant: &syn::Variant, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let variantIdent = &variant.ident;
    let fieldIdents = fields.named
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    return quote! {
        Self::#variantIdent { #( #fieldIdents, )* } => formatter
            .debug_struct(stringify!(#enumIdent))
            #( .field(stringify!(#fieldIdents), #fieldIdents) )*
            .finish()
    };
}

fn deriveDebugForUnnamedVariant(ast: &syn::DeriveInput, variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let enumIdent = &ast.ident;
    let variantIdent = &variant.ident;
    let fieldIdents = (0..fields.unnamed.len())
        .map(|index| format_ident!("arg{index}"))
        .collect::<Vec<_>>();

    return quote! {
        Self::#variantIdent(#( #fieldIdents, )*) => formatter
            .debug_tuple(stringify!(#enumIdent))
            #( .field(#fieldIdents) )*
            .finish()
    };
}

fn deriveDebugForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;

    return quote! {
        Self::#variantIdent => write!(formatter, stringify!(#variantIdent))
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
