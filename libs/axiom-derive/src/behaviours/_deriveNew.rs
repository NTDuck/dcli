use quote::format_ident;
use quote::quote;

pub fn deriveNew(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let tokens = match &ast.data {
        syn::Data::Struct(data) => deriveForStruct(&ast, data),
        _ => panic!(),
    };

    return proc_macro::TokenStream::from(tokens);
}

fn deriveForStruct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = &data.fields;

    let structIdent = &ast.ident;
    let (structImplGenerics, structTypeGenerics, structWhereClause) = ast.generics.split_for_impl();

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
                    pub fn new(#( #fieldIdents: #fieldTypes, )*) -> Self {
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
                    pub fn new(#( #fieldIdents: #fieldTypes, )*) -> Self {
                        return Self( #( #fieldIdents, )* );
                    }
                }
            };
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
    }
}
