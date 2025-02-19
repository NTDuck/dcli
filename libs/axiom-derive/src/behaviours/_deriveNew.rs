use quote::format_ident;
use quote::quote;

pub fn deriveNew(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        syn::Data::Enum(data) => deriveForEnum(&ast, &data),
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
    let methodIdent = format_ident!("{MethodName}");

    let fieldIdents = fields.named
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();
    let fieldTypes = fields.named
        .iter()
        .map(|field| &field.ty)
        .collect::<Vec<_>>();

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
    let methodIdent = format_ident!("{MethodName}");

    let fieldIdents = (0..fields.unnamed.len())
        .map(|index| format_ident!("arg{index}"))
        .collect::<Vec<_>>();
    let fieldTypes = fields.unnamed
        .iter()
        .map(|field| &field.ty)
        .collect::<Vec<_>>();

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
    let methodIdent = format_ident!("{MethodName}");

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
    let methodIdent = format_ident!("{}{}",
        MethodName, convertIdentToUpperCamelCase(variantIdent));

    let fieldIdents = fields.named
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();
    let fieldTypes = fields.named
        .iter()
        .map(|field| &field.ty)
        .collect::<Vec<_>>();

    return quote! {
        pub fn #methodIdent(#( #fieldIdents: #fieldTypes, )*) -> Self {
            return Self::#variantIdent { #(#fieldIdents, )* };
        }
    };
}

fn deriveForUnnamedVariant(variant: &syn::Variant, fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let methodIdent = format_ident!("{}{}",
        MethodName, convertIdentToUpperCamelCase(variantIdent));

    let fieldIdents = (0..fields.unnamed.len())
        .map(|index| format_ident!("arg{index}"))
        .collect::<Vec<_>>();
    let fieldTypes = fields.unnamed
        .iter()
        .map(|field| &field.ty)
        .collect::<Vec<_>>();

    return quote! {
        pub fn #methodIdent(#( #fieldIdents: #fieldTypes, )*) -> Self {
            return Self::#variantIdent( #( #fieldIdents, )* );
        }
    };
}

fn deriveForUnitVariant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variantIdent = &variant.ident;
    let methodIdent = format_ident!("{}{}",
        MethodName, convertIdentToUpperCamelCase(variantIdent));

    return quote! {
        pub fn #methodIdent() -> Self {
            return Self::#variantIdent;
        }
    };
}

fn convertIdentToUpperCamelCase(ident: &syn::Ident) -> syn::Ident {
    use heck::ToUpperCamelCase;

    return syn::Ident::new(
        ident.to_string().to_upper_camel_case().as_str(),
        ident.span(),
    );
}

const MethodName: &'static str = "new";
