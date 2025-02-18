use quote::quote;

pub fn deriveNew(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        syn::Data::Enum(data) => deriveForEnum(&ast, &data),
        syn::Data::Union(_) => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn deriveForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

    return match fields {
        syn::Fields::Named(fields) => {
            let fieldIdents: Vec<_> = fields.named
                .iter()
                .map(|field| &field.ident)
                .collect();
            let fieldTypes: Vec<_> = fields.named
                .iter()
                .map(|field| &field.ty)
                .collect();

            quote! {
                impl #structImplGenerics #structIdent #structTypeGenerics #structWhereClause {
                    pub fn new(#(#fieldIdents: #fieldTypes),*) -> Self {
                        return Self { #(#fieldIdents),* };
                    }
                }
            }
        },
        syn::Fields::Unnamed(fields) => {
            let fieldIdents: Vec<_> = (0..fields.unnamed.len())
                .map(|index| syn::Ident::new(
                    &format!("f{index}"),
                    proc_macro2::Span::call_site(),
                ))
                .collect();
            let fieldTypes: Vec<_> = fields.unnamed
                .iter()
                .map(|field| &field.ty)
                .collect();

            quote! {
                impl #structImplGenerics #structIdent #structTypeGenerics #structWhereClause {
                    pub fn new(#(#fieldIdents: #fieldTypes),*) -> Self {
                        return Self ( #(#fieldIdents),* );
                    }
                }
            }
        },
        syn::Fields::Unit => {
            return quote! {
                impl #structImplGenerics #structIdent #structTypeGenerics #structWhereClause {
                    pub fn new() -> Self {
                        return Self;
                    }
                }
            };
        },
    };
}

fn deriveForEnum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variantImpls = data.variants
        .iter()
        .map(|variant| {
            let variantIdent = &variant.ident;
            let methodIdent = syn::Ident::new(
                &format!("new{variantIdent}"),
                proc_macro2::Span::call_site(),
            );

            match &variant.fields {
                syn::Fields::Named(fields) => {
                    let fieldIdents: Vec<_> = fields.named
                        .iter()
                        .map(|field| &field.ident)
                        .collect();
                    let fieldTypes: Vec<_> = fields.named
                        .iter()
                        .map(|field| &field.ty)
                        .collect();

                    quote! {
                        pub fn #methodIdent(#(#fieldIdents: #fieldTypes),*) -> Self {
                            return Self::#variantIdent { #(#fieldIdents),* };
                        }
                    }
                },
                syn::Fields::Unnamed(fields) => {
                    let fieldIdents: Vec<_> = (0..fields.unnamed.len())
                        .map(|index| syn::Ident::new(
                            &format!("f{index}"),
                            proc_macro2::Span::call_site(),
                        ))
                        .collect();
                    let fieldTypes: Vec<_> = fields.unnamed
                        .iter()
                        .map(|field| &field.ty)
                        .collect();

                    quote! {
                        pub fn #methodIdent(#(#fieldIdents: #fieldTypes),*) -> Self {
                            return Self::#variantIdent ( #(#fieldIdents),* );
                        }
                    }
                },
                syn::Fields::Unit => {
                    let variantIdent = &variant.ident;

                    quote! {
                        pub fn #methodIdent() -> Self {
                            return Self::#variantIdent;
                        }
                    }
                },
            }
        });

    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

    return quote! {
        impl #structImplGenerics #structIdent #structTypeGenerics #structWhereClause {
            #(#variantImpls)*
        }
    };
}
