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

