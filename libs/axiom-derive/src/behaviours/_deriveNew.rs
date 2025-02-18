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

    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

    let methodIdent = format_ident!("new");

    match fields {
        syn::Fields::Named(fields) => {
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
        },
        syn::Fields::Unnamed(fields) => {
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
        },
        syn::Fields::Unit => {
            return quote! {
                impl #structImplGenerics #structIdent #structTypeGenerics #structWhereClause {
                    pub fn #methodIdent() -> Self {
                        return Self;
                    }
                }
            };
        },
    }
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    let enumIdent = &ast.ident;
    let (enumImplGenerics, enumTypeGenerics, enumWhereClause) = ast.generics.split_for_impl();

    let variantNewMethods = variants
        .iter()
        .map(|variant| {
            let variantIdent = &variant.ident;
            let methodIdent = format_ident!("new{variantIdent}");

            match &variant.fields {
                syn::Fields::Named(fields) => {
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
                },
                syn::Fields::Unnamed(fields) => {
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
                },
                syn::Fields::Unit => {
                    return quote! {
                        pub fn #methodIdent() -> Self {
                            return Self::#variantIdent;
                        }
                    };
                },
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        impl #enumImplGenerics #enumIdent #enumTypeGenerics #enumWhereClause {
            #( #variantNewMethods )*
        }
    }
}